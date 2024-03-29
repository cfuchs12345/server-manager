use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::models::{
    error::AppError,
    plugin::data::{DataDef, ResultFormat},
};

#[derive(Serialize, Deserialize)]
struct Xml {
    data: String,
}

pub fn convert_result_string_to_html(
    template: &str,
    input: String,
    template_engine: &handlebars::Handlebars<'static>,
    data: &DataDef,
) -> Result<String, AppError> {
    log::debug!("Data input is: {}", input);

    if input.is_empty() {
        log::debug!("No output to convert");
        return Ok("".to_owned());
    }

    let data_value = create_data_input_structure(data, input)?;

    format_data_with_template_engine(data_value, template_engine, template)
}

fn create_data_input_structure(data: &DataDef, input: String) -> Result<Value, AppError> {
    match &data.result_format {
        ResultFormat::XML => {
            Ok(json!(Xml {
                // wrap the xml in a JSON as content of "data" property
                data: input
            }))
        }
        ResultFormat::JSON => serde_json::from_str(input.as_str()).map_err(|e| {
            log::error!("Error while parsing JSON {}: {}", input, e);
            AppError::from(e)
        }),
    }
}

fn format_data_with_template_engine(
    data_value: Value,
    engine: &handlebars::Handlebars,
    template: &str,
) -> Result<String, AppError> {
    if data_value.is_array()
        && data_value
            .as_array()
            .ok_or(AppError::Unknown(format!(
                "Could not get array from {}",
                data_value
            )))?
            .is_empty()
    {
        log::warn!("{:?} is an array, but it is empty", data_value);
        return Ok("".to_string()); // no data input - return empty string
    }
    log::debug!("Putting data into context: {:?}", data_value);

    let res = engine.render(template, &data_value);
    res.map_err(|e| AppError::CouldNotRenderData(format!("{:?}", e)))
}
