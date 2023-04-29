use std::io::ErrorKind;

use actix_web::Error;
use serde_json::Value;

use crate::plugin_types::Data;

pub fn convert_json_to_html ( template: &str, input: String, template_engine: &handlebars::Handlebars<'static>, data: &Data) -> Result<String, Error> {
    let mut engine = template_engine.clone();

    if !data.template_helper_script.is_empty() {
        
        engine.register_script_helper_file("sort", &data.template_helper_script).unwrap();
    }

    let v: Value = serde_json::from_str(input.as_str()).unwrap();


        let res = engine.render(template, &v).map_err(|err| Error::from(std::io::Error::new(ErrorKind::Other, err)));

        if res.is_err() {
            log::error!("Error during template rendering: {:?}", &res);
        }
        res
}