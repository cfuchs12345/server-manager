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

    let data_value: Value = match &data.result_format {
        ResultFormat::XML => {
            json!(Xml {
                data: input
            })
        },
        _ => {
            serde_json::from_str(input.as_str()).unwrap() 
        }
    }; 
    
    log::info!("Putting data into context: {:?}", data_value);

    let res = engine
        .render(template, &data_value)
        .map_err(|err| Error::from(std::io::Error::new(ErrorKind::Other, err)));  

    if res.is_err() {
        log::error!("Error during template rendering: {:?}", &res);
    }
    res
}
