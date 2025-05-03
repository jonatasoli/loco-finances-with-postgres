use loco_rs::prelude::*;

use crate::models::_entities::transactions;

/// Render a list view of `transactions`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<transactions::Model>) -> Result<Response> {
    format::render().view(v, "transaction/list.html", data!({"items": items}))
}

/// Render a single `transaction` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &transactions::Model) -> Result<Response> {
    format::render().view(v, "transaction/show.html", data!({"item": item}))
}

/// Render a `transaction` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "transaction/create.html", data!({}))
}

/// Render a `transaction` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &transactions::Model) -> Result<Response> {
    format::render().view(v, "transaction/edit.html", data!({"item": item}))
}
