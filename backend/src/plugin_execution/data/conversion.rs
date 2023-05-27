use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::models::{
    error::AppError,
    plugin::data::{Data, ResultFormat},
};

#[derive(Serialize, Deserialize)]
struct Xml {
    data: String,
}

pub fn convert_result_string_to_html(
    template: &str,
    input: String,
    template_engine: &handlebars::Handlebars<'static>,
    data: &Data,
) -> Result<String, AppError> {
    log::debug!("Data input is: {}", input);

    let data_value = create_data_input_structure(data, input)?;

    format_data_with_template_engine(data_value, template_engine, template)
}

fn create_data_input_structure(data: &Data, input: String) -> Result<Value, AppError> {
    match &data.result_format {
        ResultFormat::XML => { 
            Ok(json!(Xml {
                // wrap the xml in a JSON as content of "data" property
                data: input
            }))
        }
        ResultFormat::JSON => serde_json::from_str(input.as_str()).map_err(AppError::from)
    }
}

fn format_data_with_template_engine(
    data_value: Value,
    engine: &handlebars::Handlebars,
    template: &str,
) -> Result<String, AppError> {
    if data_value.is_array() && data_value.as_array().unwrap().is_empty() {
        return Ok("".to_string()); // no data input - return empty string
    }
    log::debug!("Putting data into context: {:?}", data_value);

    let res = engine.render(template, &data_value);
    res.map_err(|e| AppError::CouldNotRenderData(format!("{:?}", e)))
    
}
