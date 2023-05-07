use crate::persistence;

#[derive(Debug, Clone)]
pub struct AppData {
    pub app_data_persistence: persistence::Persistence,
    pub app_data_template_engine: handlebars::Handlebars<'static>
}