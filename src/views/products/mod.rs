

mod new;
mod index;
mod edit;

use crate::Route;
use crate::auth::AuthContext;
use crate::components::{ButtonLink, ButtonVariant};
use dioxus::prelude::*;

pub use edit::EditProduct;
pub use index::IndexProducts;
pub use new::NewProduct;

#[component]
pub fn ProductAuthGuard() -> Element {
	let auth = use_context::<AuthContext>();

	if auth.session().is_none() {
		return rsx! {
			div { class: "flex min-h-[70vh] flex-col items-center justify-center gap-4 px-6 text-center",
				h1 { class: "text-2xl font-bold text-zinc-900 dark:text-zinc-100", "Sign in required" }
				p { class: "max-w-md text-zinc-500 dark:text-zinc-400", "Sign in to view and manage your products." }
				ButtonLink { to: Route::Login {}, variant: ButtonVariant::Primary, "Sign in" }
			}
		};
	}

	rsx! { Outlet::<Route> {} }
}