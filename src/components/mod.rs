//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.
mod auth_form;
pub use auth_form::AuthForm;

mod product;
pub use product::ProductFormView;

mod contact;
pub use contact::ContactFormView;

mod invoice;
pub use invoice::{InvoiceDraft, InvoiceFormView};

mod table;
pub use table::{Table, TableBody, TableCell, TableHead, TableHeaderCell, TableRow};
pub use table::{TableInput, TableSelect};
pub mod input;
pub mod label;
pub use input::{AuthLink, Button, ButtonLink, ButtonVariant, FormInput};
pub mod textarea;

mod searchbar;
pub use searchbar::SearchBar;
