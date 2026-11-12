//! Server-only authentication backend for the shared `#[server]` functions.

use std::sync::{Arc, LazyLock};

use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, PasswordVerifier};
use axum_extra::extract::SignedCookieJar;
use chrono::{TimeDelta, Utc};
use cookie::{Cookie, Expiration, Key};
use dioxus::fullstack::extract::{FromRef, FromRequestParts};
use dioxus::fullstack::http::HeaderValue;
use dioxus::fullstack::http::header::SET_COOKIE;
use dioxus::fullstack::response::IntoResponse;
use dioxus::fullstack::{FullstackContext, HeaderMap};
use dioxus::logger::tracing::warn;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::db::establish_connection;
use crate::api::auth::Session;
use crate::models::{User, UserForm};
use crate::server::schema::users;

/// How long a login session stays valid after it is issued.
const SESSION_TTL_DAYS: i64 = 7;

/// The name of the cookie used to carry the signed session payload.
const SESSION_COOKIE: &str = "nano_pos_session";

/// The signed payload embedded inside the session cookie.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct SessionPayload {
    uid: i32,
    /// Session expiry as a unix timestamp (seconds).
    exp: i64,
}

impl SessionPayload {
    /// Check if the session has expired.
    fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }

    fn for_user(user_id: i32) -> Self {
        let expires = Utc::now() + TimeDelta::days(SESSION_TTL_DAYS);
        Self {
            uid: user_id,
            exp: expires.timestamp(),
        }
    }

    fn cookie(&self) -> Result<Cookie<'static>, ServerFnError> {
        serde_json::to_string(self)
            .map(|value| {
                Cookie::build((SESSION_COOKIE, value))
                    .http_only(true)
                    .secure(true)
                    .same_site(cookie::SameSite::Strict)
                    .path("/")
                    .expires(Expiration::Session)
                    .into()
            })
            .map_err(|error| ServerFnError::Response(format!("Could not create session: {error}")))
    }
}

/// The session-signing secret, derived once for the process lifetime.
static SESSION_SECRET: LazyLock<Arc<[u8; 32]>> = LazyLock::new(load_secret);

fn load_secret() -> Arc<[u8; 32]> {
    if let Err(e) = dotenvy::dotenv() {
        warn!("Failed to load .env file: {e}");
    }
    // A fixed `SESSION_SECRET` keeps sessions valid across server restarts.
    // When it is absent we derive a pseudo-random key from the environment so
    // sessions do not survive a restart. Configure `SESSION_SECRET` in
    // production for stable, shareable sessions.
    let secret: [u8; 32] = match std::env::var("SESSION_SECRET") {
        Ok(value) if !value.is_empty() => {
            let mut buf = [0u8; 32];
            let bytes = value.as_bytes();
            let n = bytes.len().min(32);
            buf[..n].copy_from_slice(&bytes[..n]);
            buf
        }
        _ => {
            warn!(
                "SESSION_SECRET is not set; derived an ephemeral signing key. \
                 Sessions will be invalidated on server restart."
            );
            derive_ephemeral_secret()
        }
    };
    Arc::new(secret)
}

/// Mix a few low-entropy, per-process sources through SHA-256 to produce a
/// development signing key. Not cryptographically strong; set `SESSION_SECRET`
/// in production.
fn derive_ephemeral_secret() -> [u8; 32] {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seed = format!(
        "{nanos}-{}-{}-{}",
        std::process::id(),
        std::env::args().next().unwrap_or_default(),
        heap_entropy()
    );
    let hash = Sha256::digest(seed.as_bytes());
    let mut buf = [0u8; 32];
    buf.copy_from_slice(hash.as_slice());
    buf
}

/// A cheap per-process entropy value derived from the address of a heap
/// allocation (ASLR varies this run to run). The allocation is freed, so this
/// leaks nothing.
fn heap_entropy() -> usize {
    let boxed = Box::new(0u8);
    let ptr = Box::into_raw(boxed);
    drop(unsafe { Box::from_raw(ptr) });
    ptr as usize
}

/// Hash a plaintext password with Argon2. A random salt is generated per hash
/// (the `password-hash` crate fills the salt automatically).
fn hash_password(password: &str) -> Result<String, ServerFnError> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|e| ServerFnError::Response(format!("Failed to hash password: {e}")))
}

/// Verify a plaintext password against a stored Argon2 hash.
fn verify_password(password: &str, hash: &str) -> bool {
    Argon2::default()
        .verify_password(password.as_bytes(), hash)
        .is_ok()
}

#[cfg(test)]
#[test]
fn test_hash_and_verify() {
    let password = "correct horse battery staple";
    let hash = hash_password(password).expect("hashing should succeed");
    assert!(verify_password(password, &hash));
    assert!(!verify_password("wrong password", &hash));
}

#[derive(Clone, Debug, PartialEq)]
pub struct SessionKey(Key);

impl FromRef<FullstackContext> for SessionKey {
    fn from_ref(input: &FullstackContext) -> Self {
        input.extension::<SessionKey>().unwrap()
    }
}

pub struct SessionCookieJar {
    jar: SignedCookieJar,
}

impl SessionCookieJar {
    fn user_id(&self) -> Option<i32> {
        let cookie = self.jar.get(SESSION_COOKIE)?;
        let payload = serde_json::from_str::<SessionPayload>(cookie.value()).ok()?;
        (!payload.is_expired()).then_some(payload.uid)
    }

    pub fn session(&self) -> Result<Session, ServerFnError> {
        let uid = self.user_id().ok_or_else(|| {
            ServerFnError::Response("You must be signed in to access this resource.".into())
        })?;
        load_session(uid)
    }

    pub fn set_session(mut self, user_id: i32) -> Result<Self, ServerFnError> {
        let cookie = SessionPayload::for_user(user_id).cookie()?;
        self.jar = self.jar.add(cookie);
        self.commit_response()?;
        Ok(self)
    }

    fn commit_response(&self) -> Result<(), ServerFnError> {
        let response = self.jar.clone().into_response();
        for value in response.headers().get_all(SET_COOKIE).iter() {
            FullstackContext::current()
                .ok_or_else(|| ServerFnError::Response("No active request context.".into()))?
                .add_response_header(SET_COOKIE, value.clone());
        }
        Ok(())
    }
}

impl<S> FromRequestParts<S> for SessionCookieJar
where
    S: Send + Sync,
    SessionKey: FromRef<S>,
{
    type Rejection = <HeaderMap as FromRequestParts<S>>::Rejection;
    async fn from_request_parts(
        parts: &mut dioxus_server::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let map = HeaderMap::from_request_parts(parts, state).await?;
        let key = SessionKey::from_ref(state);
        let jar = SignedCookieJar::from_headers(&map, key.0);
        Ok(Self { jar })
    }
}

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
    SessionCookieJar: FromRequestParts<S>,
{
    type Rejection = dioxus::fullstack::http::StatusCode;

    async fn from_request_parts(
        parts: &mut dioxus_server::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let jar = SessionCookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| dioxus::fullstack::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        jar.session()
            .map_err(|_| dioxus::fullstack::http::StatusCode::UNAUTHORIZED)
    }
}

pub(crate) fn session_key() -> SessionKey {
    let mut key = [0u8; 64];
    key[..SESSION_SECRET.len()].copy_from_slice(SESSION_SECRET.as_slice());
    SessionKey(Key::from(&key))
}

/// Attach a `Set-Cookie` header (through the active request context) that
/// establishes a session for `user_id`. The browser (same-origin fetch on web,
/// reqwest cookie jar on desktop) stores the HttpOnly cookie.
/// Attach a `Set-Cookie` header that expires (clears) the session cookie.
fn clear_session_cookie() {
    let cookie = cookie::Cookie::build(SESSION_COOKIE)
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Strict)
        .secure(true)
        .removal();
    if let Some(ctx) = FullstackContext::current() {
        let Ok(value) = HeaderValue::from_str(&cookie.to_string()) else {
            warn!("Failed to create Set-Cookie header to clear session cookie");
            return;
        };
        ctx.add_response_header(SET_COOKIE, value);
        info!("Cleared session cookie for request");
    }
}

/// Build a [`Session`] from a database [`User`] row.
fn session_from_user(user: &User) -> Session {
    Session {
        id: user.id,
        username: user.username.clone(),
    }
}

/// Register a new user, set the session cookie, and return the new session.
pub fn signup_impl(
    username: &str,
    email: &str,
    password: &str,
    jar: SessionCookieJar,
) -> Result<Session, ServerFnError> {
    use diesel::prelude::*;

    let username = username.trim();
    let email = email.trim();
    let password = password.trim();
    if username.is_empty() || email.is_empty() || password.is_empty() {
        return Err(ServerFnError::new(
            "Username, email, and password are required.",
        ));
    }
    if !email.contains('@') {
        return Err(ServerFnError::new("Email must contain '@'."));
    }
    if password.len() < 8 {
        return Err(ServerFnError::new(
            "Password must be at least 8 characters.",
        ));
    }

    // Hash before touching the DB connection so the lock is only held briefly.
    let password_hash = hash_password(password)?;

    let user = {
        let mut conn = establish_connection();
        diesel::insert_into(users::table)
            .values(&UserForm {
                username,
                password_hash: &password_hash,
                email,
            })
            .get_result::<User>(&mut *conn)
            .map_err(|e| {
                ServerFnError::Response(format!(
                    "Could not create user (username or email may already be taken): {e}"
                ))
            })?
    };

    let session = session_from_user(&user);
    let _ = jar.set_session(session.id)?;
    Ok(session)
}

/// Log a user in by username and password, setting the session cookie.
pub fn login_impl(
    username: &str,
    password: &str,
    jar: SessionCookieJar,
) -> Result<Session, ServerFnError> {
    use diesel::prelude::*;

    let username = username.trim().to_string();
    let password = password.trim();
    let user = {
        let mut conn = establish_connection();
        users::table
            .filter(users::username.eq(&username))
            .first::<User>(&mut *conn)
            .optional()
            .map_err(|e| ServerFnError::Response(format!("Database error: {e}")))?
    };

    let user = user.ok_or_else(|| ServerFnError::Response("Invalid credentials".into()))?;
    if !verify_password(password, &user.password_hash) {
        return Err(ServerFnError::Response("Invalid credentials".into()));
    }

    let session = session_from_user(&user);
    let _ = jar.set_session(session.id)?;
    Ok(session)
}

fn load_session(uid: i32) -> Result<Session, ServerFnError> {
    use diesel::prelude::*;

    let mut conn = establish_connection();
    let user = User::query()
        .filter(users::id.eq(uid))
        .first(&mut *conn)
        .optional()
        .map_err(|e| ServerFnError::Response(format!("Database error: {e}")))?
        .ok_or_else(|| {
            ServerFnError::Response("You must be signed in to access this resource.".into())
        })?;
    Ok(session_from_user(&user))
}

/// Clear the caller's session cookie.
pub fn logout_impl() {
    clear_session_cookie();
}
