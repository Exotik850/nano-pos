//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`Home`] and [`Blog`] components will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod home;
pub use home::Home;

mod navbar;
pub use navbar::Navbar;

mod login;
pub use login::{Login, LoginForm};

mod signup;
pub use signup::{Signup, SignupForm};

mod sidebar;
pub use sidebar::Sidebar;

mod products;
pub use products::{EditProduct, IndexProducts, NewProduct};

mod contacts;
pub use contacts::{EditContact, IndexContacts, NewContact};

mod invoices;
pub use invoices::{EditInvoice, IndexInvoices, NewInvoice};
