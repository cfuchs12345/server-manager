mod functions;

use crate::{datastore, models::error::AppError};
use handlebars::no_escape;

pub fn create_templateengine() -> Result<handlebars::Handlebars<'static>, AppError> {
    let config = datastore::get_config()?;
    let template_base_path = config.get_string("template_base_path")?;

    log::debug!("dir: {}", template_base_path);

    configure(&template_base_path)
}

fn configure(template_base_path: &str) -> Result<handlebars::Handlebars<'static>, AppError> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(".html", template_base_path)
        .map_err(|err| {
            AppError::Unknown(format!(
                "Could not register directoy for templates. Error {}",
                err
            ))
        })?;
    handlebars.set_dev_mode(true);

    let templates = handlebars.get_templates();
    for entry in templates {
        log::debug!("registered template: {}", entry.0);
    }

    handlebars.set_strict_mode(true);
    handlebars.register_escape_fn(no_escape); //html escaping is the default and cause issue
    functions::register(&mut handlebars);

    Ok(handlebars)
}
