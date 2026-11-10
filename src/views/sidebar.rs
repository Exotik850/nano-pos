use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronsLeft;

use crate::components::{Button, ButtonVariant};
use crate::Route;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Hidden(pub Signal<bool>);

#[component]
pub fn SidebarLink(
  #[props(into)] to: NavigationTarget,
  children: Element,
) -> Element {
  rsx! {
    Link {
        class: "rounded-lg px-3 py-2 text-sm font-medium text-zinc-700 transition-colors duration-200 ease-out hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800",
        to,
        {children}
    }
}

}

#[component]
pub fn Sidebar() -> Element {
    let auth = use_context::<crate::auth::AuthContext>();
    let mut hidden = use_signal(|| true);
    use_context_provider(|| Hidden(hidden));

    if auth.session().is_none() {
       return rsx! {
        div { class: "flex min-h-screen w-full",
            main { class: "min-w-0 flex-1", Outlet::<Route> {} }
        }
    };
    }

    rsx! {
        div { class: "flex min-h-screen w-full transition-all duration-300",
            aside {
                class: "flex w-56 shrink-0 flex-col gap-6 border-r border-zinc-200 px-4 py-4 dark:border-zinc-700 transition-all duration-300 ease-in-out",
                class: if hidden() { "pointer-events-none !w-0 -translate-x-full overflow-hidden !border-0 !px-0 opacity-0" } else { "translate-x-0 opacity-100" },
                div { class: "flex items-center justify-between",
                    h2 { class: "text-sm font-semibold uppercase tracking-wide text-zinc-500 dark:text-zinc-400",
                        "Workspace"
                    }
                    Button {
                        class: "px-2 py-1 text-xs",
                        variant: ButtonVariant::Secondary,
                        r#type: "button",
                        onclick: move |_| hidden.set(true),
                        ChevronsLeft { size: 16 }
                    }
                }
                nav {
                    class: "flex flex-col gap-1",
                    "aria-label": "Main navigation",
                    SidebarLink { to: Route::Home {}, "Home" }
                    SidebarLink { to: Route::IndexProducts {}, "Products" }
                    SidebarLink { to: Route::IndexContacts {}, "Contacts" }
                    SidebarLink { to: Route::IndexInvoices {}, "Invoices" }
                }
            }
            main { class: "min-w-0 flex-1", Outlet::<Route> {} }
        }
    }
}