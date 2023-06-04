use crate::datastore::{Persistence, TimeSeriesPersistence};

#[derive(Debug, Clone)]
pub struct AppData {
    pub app_data_persistence: Persistence,
    pub app_data_timeseries_persistence: TimeSeriesPersistence,
    pub app_data_template_engine: handlebars::Handlebars<'static>,
}
