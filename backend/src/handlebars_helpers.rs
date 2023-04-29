use std::cmp::Ordering;
use serde_json::Value as Json;
use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, handlebars_helper};

fn sort_fct(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    rc: &mut RenderContext,
    _: &mut dyn Output,
) -> HelperResult {
    // get parameter from helper or throw an error
    let name_option = h.param(0).and_then(|v| v.value().as_str());
    if name_option.is_none() {
        return Ok(());
    }

    let object_value = h.param(1).and_then(|v| v.value().as_str());
    if object_value.is_none() {
        return Ok(());
    }

    let name = name_option.unwrap();
    
    match ctx.data().get(name) {
        Some(value) => {
            if value.is_array() {
                let property = object_value.unwrap();

                let mut sorted_array = value.as_array().unwrap().to_owned();
                
                
                sorted_array.sort_by( |a, b| {
                    let a_prop = a[property].as_str();
                    let b_prop = b[property].as_str();

                    if a_prop.is_some() && b_prop.is_some() { // string property
                        let a_i64 =  a_prop.unwrap().parse::<i64>();
                        let b_i64 =  b_prop.unwrap().parse::<i64>();

                        if a_i64.is_ok() && b_i64.is_ok() {
                            a_i64.unwrap().partial_cmp(&b_i64.unwrap()).unwrap()
                        }
                        else { // normal string sort
                            a_prop.unwrap().partial_cmp(b_prop.unwrap()).unwrap()
                        }                    
                    }
                    else { // numeric property
                        let a_prop = a[property].as_i64();
                        let b_prop = b[property].as_i64();
    
                        if a_prop.is_some() && b_prop.is_some() {
                            a_prop.unwrap().partial_cmp(&b_prop.unwrap()).unwrap()
                        }
                        else {                            
                            log::error!("Properties with name {} has an unknown type that cannot be sorted", property);
                            Ordering::Less
                        }
                    }                
                });

                let str = serde_json::to_string(&sorted_array).unwrap();

                let mut ctx = ctx.clone();
                match ctx.data_mut() {
                    serde_json::value::Value::Object(m) => m.insert(name.to_owned(), serde_json::from_str(str.as_str()).unwrap()),
                    _ => None,
                };
                rc.set_context(ctx);

            }
            else {
                log::error!("Value with name {} is not an array", name);
                return Ok(())
            }
        },
        None => {
            log::error!("Value with name {} not found", name);
            return Ok(())
        }
    };


  
    Ok(())
}








handlebars_helper!(readable_time: |secs: Json| to_readable_time(secs));
handlebars_helper!(readable_mem: |mem: Json| to_readable_mem(mem));

pub fn register(handlebars: &mut Handlebars) {
    handlebars.register_helper("readable-time", Box::new(readable_time));
    handlebars.register_helper("readable-mem", Box::new(readable_mem));
    handlebars.register_helper("sort", Box::new(sort_fct))
}

fn to_readable_mem( value: &Json) -> String {
    if value.is_i64() {
        match value.as_i64() {
            Some(val) => {
                let mb = val / 1024 / 1024;
                
                format!("{} MB", mb)
            },
            None => {
                value.as_str().unwrap().to_owned()        
            }
        }
    }
    else {
        value.as_str().unwrap().to_owned()
    }
}

fn to_readable_time( value: &Json) -> String {
    if value.is_i64() {
        match value.as_i64() {
            Some(val) => {
                let days = val / (3600*24);
                let hours  = val % (3600*24) /3600;
                let minutes = val % 3600 / 60;
                let seconds = val % 60;
                
                format!("{}d {:02}h {:02}m {:02}s", days, hours, minutes, seconds)
            },
            None => {
                value.as_str().unwrap().to_owned()        
            }
        }
    }
    else {
        value.as_str().unwrap().to_owned()
    }
}