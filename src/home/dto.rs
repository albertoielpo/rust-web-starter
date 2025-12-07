use serde::Serialize;

/// Data transfer object for the home page.
///
/// Contains the view model data that will be rendered
/// in the `home.hbs` Handlebars template.
#[derive(Serialize)]
pub struct HomeData {
    /// ISO 8601 formatted
    pub first_hit: String,

    // Title
    pub title: String,
}
