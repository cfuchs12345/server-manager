mod functions;

use crate::datastore;
use handlebars::no_escape;

pub fn create_templateengine() -> handlebars::Handlebars<'static> {
    let config = datastore::get_config();
    let template_base_path = config.get_string("template_base_path").unwrap();

    log::debug!("dir: {}", template_base_path);

    create_and_configure_template_engine(&template_base_path)
}



fn create_and_configure_template_engine(
    template_base_path: &str,
) -> handlebars::Handlebars<'static> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(".html", template_base_path)
        .unwrap();
    handlebars.set_dev_mode(true);

    let templates = handlebars.get_templates();
    for entry in templates {
        log::debug!("registered template: {}", entry.0);
    }

    handlebars.set_strict_mode(true);
    handlebars.register_escape_fn(no_escape); //html escaping is the default and cause issue
    functions::register(&mut handlebars);

    handlebars
}
