use dioxus::prelude::*;

use crate::Route;
use crate::components::{ButtonLink, ButtonVariant, DocumentForm, ProductLineDraft};
use crate::models::{
    Contact, EstimateDetails, EstimateDraft, EstimateForm, EstimateProductForm, Product,
};

#[component]
pub fn EstimateFormView(
    #[props(default)] initial: Option<EstimateDetails>,
    contacts: Vec<Contact>,
    products: Vec<Product>,
    submit_label: String,
    onsubmit: EventHandler<Result<EstimateDraft, String>>,
) -> Element {
    let customer_id = use_signal(|| {
        initial
            .as_ref()
            .and_then(|value| value.estimate.customer_id)
    });
    let doc_number = use_signal(|| {
        initial
            .as_ref()
            .map_or_else(String::new, |value| value.estimate.doc_number.clone())
    });
    let description = use_signal(|| {
        initial
            .as_ref()
            .and_then(|value| value.estimate.description.clone())
            .unwrap_or_default()
    });
    let lines = use_signal(|| {
        initial
            .as_ref()
            .map(|value| {
                value
                    .products
                    .iter()
                    .map(|line| ProductLineDraft {
                        product_id: line.product_id,
                        quantity: line.quantity,
                        price: line.price,
                    })
                    .collect()
            })
            .unwrap_or_default()
    });

    rsx! {
        DocumentForm {
            doc_number,
            customer_id,
            contacts,
            description,
            products,
            lines,
            heading: "Estimate products",
            line_description: "Attach products and quantities to this estimate.",
            description_placeholder: "Add notes for this estimate",
            submit_label,
            action: rsx! {
                ButtonLink {
                    to: Route::IndexProducts {},
                    variant: ButtonVariant::Secondary,
                    class: "px-3 py-1.5 text-sm",
                    "Manage products"
                }
            },
            onsubmit: move |_| {
                if doc_number().trim().is_empty() {
                    onsubmit.call(Err("Document number is required.".to_string()));
                    return;
                }
                onsubmit
                    .call(
                        Ok(EstimateDraft {
                            estimate: EstimateForm {
                                customer_id: customer_id(),
                                user_id: 0,
                                doc_number: doc_number(),
                                description: (!description().is_empty()).then(&*description),
                            },
                            products: lines()
                                .into_iter()
                                .map(|line| EstimateProductForm {
                                    estimate_id: 0,
                                    product_id: line.product_id,
                                    quantity: line.quantity,
                                    price: line.price,
                                    tax_rate: None,
                                    description: None,
                                })
                                .collect(),
                        }),
                    );
            }
        }
    }
}
