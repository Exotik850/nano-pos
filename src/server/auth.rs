//! Server-only authentication backend for the shared `#[server]` functions.

use std::sync::{Arc, LazyLock};

use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, PasswordVerifier};
use chrono::{TimeDelta, Utc};
use cookie::{Cookie, CookieJar, Key};
use dioxus::fullstack::http::HeaderValue;
use dioxus::fullstack::http::header::{COOKIE, SET_COOKIE};
use dioxus::fullstack::{FullstackContext, HeaderMap};
use dioxus::logger::tracing::warn;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::db::establish_connection;
use super::models::User;
use super::schema::users;
use crate::auth::Session;
use crate::server::models::UserForm;

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

fn session_key() -> Key {
    let mut key = [0u8; 64];
    key[..SESSION_SECRET.len()].copy_from_slice(SESSION_SECRET.as_slice());
    Key::from(&key)
}

/// Build a signed session cookie value for `user_id`, valid for
/// `SESSION_TTL_DAYS`.
fn create_session_cookie(user_id: i32) -> String {
    let expires = Utc::now() + TimeDelta::days(SESSION_TTL_DAYS);
    let payload = SessionPayload {
        uid: user_id,
        exp: expires.timestamp(),
    };
    let mut jar = CookieJar::new();
    jar.signed_mut(&session_key()).add(Cookie::new(
        SESSION_COOKIE,
        serde_json::to_string(&payload).expect("SessionPayload is serializable"),
    ));
    jar.get(SESSION_COOKIE).unwrap().value().to_string()
}

#[cfg(test)]
#[test]
fn test_session_cookie_round_trip() {
    let cookie = create_session_cookie(42);
    assert_eq!(parse_session_cookie(&cookie), Some(42));
}

/// Validate a session cookie value, returning the user id when the signature
/// checks out and the session has not expired.
fn parse_session_cookie(value: &str) -> Option<i32> {
    let mut jar = CookieJar::new();
    jar.add_original(Cookie::parse(format!("{SESSION_COOKIE}={value}")).ok()?);
    let payload: SessionPayload =
        serde_json::from_str(jar.signed(&session_key()).get(SESSION_COOKIE)?.value()).ok()?;
    if Utc::now().timestamp() > payload.exp {
        return None;
    }
    Some(payload.uid)
}

/// Read the session cookie value out of the incoming request headers.
fn read_session_cookie(headers: &HeaderMap) -> Option<String> {
    let raw = headers.get(COOKIE)?.to_str().ok()?;
    raw.split(';').find_map(|pair| {
        let (name, val) = pair.trim().split_once('=')?;
        (name == SESSION_COOKIE).then(|| val.to_string())
    })
}

/// Attach a `Set-Cookie` header (through the active request context) that
/// establishes a session for `user_id`. The browser (same-origin fetch on web,
/// reqwest cookie jar on desktop) stores the HttpOnly cookie.
fn set_session_cookie(user_id: i32) {
    let value = create_session_cookie(user_id);
    let expires = Utc::now() + TimeDelta::days(SESSION_TTL_DAYS);
    let cookie = format!(
        "{SESSION_COOKIE}={value}; Path=/; HttpOnly; SameSite=Strict; Max-Age={}; Expires={}",
        SESSION_TTL_DAYS * 24 * 60 * 60,
        expires.format("%a, %d %b %Y %H:%M:%S GMT")
    );
    if let Some(ctx) = FullstackContext::current()
        && let Ok(value) = HeaderValue::from_str(&cookie)
    {
        ctx.add_response_header(SET_COOKIE, value);
    }
}

/// Attach a `Set-Cookie` header that expires (clears) the session cookie.
fn clear_session_cookie() {
    let cookie = format!(
        "{SESSION_COOKIE}=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0; \
         Expires=Thu, 01 Jan 1970 00:00:00 GMT"
    );
    if let Some(ctx) = FullstackContext::current() {
        let Ok(value) = HeaderValue::from_str(&cookie) else {
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
pub fn signup_impl(username: &str, email: &str, password: &str) -> Result<Session, ServerFnError> {
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
    set_session_cookie(session.id);
    Ok(session)
}

/// Log a user in by username and password, setting the session cookie.
pub fn login_impl(username: &str, password: &str) -> Result<Session, ServerFnError> {
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
    if !verify_password(&password, &user.password_hash) {
        return Err(ServerFnError::Response("Invalid credentials".into()));
    }

    let session = session_from_user(&user);
    set_session_cookie(session.id);
    Ok(session)
}

/// Load the user referenced by the session cookie, if the cookie is valid.
pub fn current_user_impl(headers: &HeaderMap) -> Result<Option<Session>, ServerFnError> {
    use diesel::prelude::*;

    let Some(raw) = read_session_cookie(headers) else {
        return Ok(None);
    };
    let Some(uid) = parse_session_cookie(&raw) else {
        return Ok(None);
    };

    let user = {
        let mut conn = establish_connection();
        users::table
            .filter(users::id.eq(uid))
            .first::<User>(&mut *conn)
            .optional()
            .map_err(|e| ServerFnError::Response(format!("Database error: {e}")))?
    };

    Ok(user.map(|u| session_from_user(&u)))
}

/// Clear the caller's session cookie.
pub fn logout_impl() {
    clear_session_cookie();
}
