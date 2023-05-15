mod conversion;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{models::{server::{Server, Feature}, error::AppError, plugin::{Plugin, data::Data, sub_action::SubAction}, input::ActionOrDataInput, response::data_result::DataResult}, commands, datastore, plugin_execution::actions};

lazy_static! {
    static ref TEMPLATE_SUB_ACTION_REGEX: Regex = Regex::new(r"(\[\[Action .*?\]\])").unwrap();
}


/// Executes all data queries on the given server for all given plugins
/// # Arguments
///
/// * `server` - the server struct representing the server on which the query should be executed
/// * `template_engine` - the template engine struct that is used to render the output in a readable format
/// * `persistence` - the persistence struct that helps to interact with the underlying database
pub async fn execute_data_query(
    server: &Server,
    template_engine: &handlebars::Handlebars<'static>,
    crypto_key: &str,
) -> Result<Vec<DataResult>, AppError> {
    let mut results: Vec<DataResult> = vec![];

    for feature in &server.features {
        let plugin_opt = datastore::get_plugin(feature.id.as_str());

        if plugin_opt.is_none() {
            log::error!("plugin {} not found", feature.id);
            continue;
        }
        let plugin = plugin_opt.unwrap();

        for data in &plugin.data {
            log::debug!("Plugin data execute {} {}", plugin.id, data.id);

            if !data.output {
                log::debug!("Skipping data entry {} of plugin {} since it is marked as output = false", data.id, plugin.id);
                continue;
            }
            let data_response =
                execute_specific_data_query(server, &plugin, feature, data, None, crypto_key)
                    .await?;



            
            if let Some(response) = data_response {
                let result = if !data.template.is_empty() {
                    // convert the output with the template
                    conversion::convert_json_to_html(
                        data.template.as_str(),
                        response,
                        template_engine,
                        data,
                    )?

                } else {
                    // no template - just append
                    response
                };

                let enriched_result = inject_meta_data_for_actions(result, feature, data);

                let actions = extract_actions(&enriched_result);
                
                let check_results = actions::check_action_conditions(server, actions, crypto_key).await;
                log::info!("-- {:?}", check_results);
                results.push(DataResult{
                    ipaddress: server.ipaddress.clone(),
                    result: enriched_result,
                    check_results
                });
            }
        }
    }

    Ok(results)
}


/// Executes a specific data query on the given server for a given data point config of a plugin
/// # Arguments
///
/// * `server` - the server struct representing the server on which the query should be executed
/// * `plugin` - the plugin to which the data query belongs to
/// * `feature` - server feature config of the specific server containing maybe additional parameters or required credentials for the server
/// * `data` - the actual data query (as defined in the plugin) that should be executed
/// * `persistence` - the persistence struct that helps to interact with the underlying database
pub async fn execute_specific_data_query(
    server: &Server,
    plugin: &Plugin,
    feature: &Feature,
    data: &Data,
    action_params: Option<&str>,
    crypto_key: &str,
) -> Result<Option<String>, AppError> {
    let input: ActionOrDataInput =
        ActionOrDataInput::get_input_from_data(data, action_params, plugin, feature, crypto_key);

    commands::execute_command(Some(server.ipaddress.clone()), &input).await
}




fn extract_actions(input: &str) -> Vec<SubAction> {
    let mut result = Vec::new();
    let groups: Vec<String> = TEMPLATE_SUB_ACTION_REGEX.find_iter(input).map(|mat| mat.as_str().to_owned()).collect();
    for group in groups {
        result.push(SubAction::from(group));
    }
    
    result
}


fn inject_meta_data_for_actions(input : String, feature: &Feature, data: &Data) -> String {    
    input.replace("[[Action ", format!("[[Action feature.id=\"{}\" data.id=\"{}\" ", feature.id, data.id).as_str())
}