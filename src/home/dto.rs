use serde::Serialize;

/// Data transfer object for the home page.
///
/// Contains the view model data that will be rendered
/// in the `index.hbs` Handlebars template.
#[derive(Serialize)]
pub struct IndexData {
    /// ISO 8601 formatted current timestamp
    pub iso_date: String,
    /// Value retrieved from Redis cache
    pub mykey: String,
}
