use crate::datastore::Persistence;

#[derive(Debug, Clone)]
pub struct AppData {
    pub app_data_persistence: Persistence,
    pub app_data_template_engine: handlebars::Handlebars<'static>,
}
