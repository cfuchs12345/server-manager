use std::io::ErrorKind;

use actix_web::Error;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use crate::plugin_types::{Data, ResultFormat};

#[derive(Serialize, Deserialize)]
struct Xml {
    data: String
}

pub fn convert_json_to_html(
    template: &str,
    input: String,
    template_engine: &handlebars::Handlebars<'static>,
    data: &Data,
) -> Result<String, Error> {
    let engine = template_engine.clone();

    let data_value: Option<Value> = match &data.result_format {
        ResultFormat::XML => {
            Some(json!(Xml { // wrap the xml in a JSON as content of "data" property
                data: input
            }))
        },
        _ => {
            match serde_json::from_str(input.as_str()) {
                Ok(val) => Some(val),
                Err(err) => {
                    if !input.trim().is_empty() {
                        log::error!("input '{}' was no valid json. Resulted in the following error: {}", input, err);
                    }                        
                    None
                }
            }
        }
    }; 

    let res_string = if let Some(data) = data_value {
        log::debug!("Putting data into context: {:?}", data);

        let result = engine
            .render(template, &data)
            .map_err(|err| Error::from(std::io::Error::new(ErrorKind::Other, err)));  


        if let Ok(rendered) = result {
            rendered
        }
        else {
            log::error!("Error during template rendering: {:?}", &result);
            "".to_string()
        }
    }
    else {
        "".to_string()
    };
    Ok(res_string)
}
