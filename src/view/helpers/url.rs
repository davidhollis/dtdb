use std::{collections::HashMap, fmt::Display, error::Error};

use serde_json::Value;

use crate::util;

pub fn url(args: &HashMap<String, Value>) -> tera::Result<Value> {
    let id_str = extract_id(args)?;
    let id_prefix = extract_id_prefix(id_str)?;
    let route = util::route_for_id_prefix(id_prefix);

    match extract_action(args) {
        "view" => Ok(Value::String(format!("{route}/{id_str}"))),
        action => Ok(Value::String(format!("{route}/{id_str}/{action}"))),
    }
}

pub fn index_url(args: &HashMap<String, Value>) -> tera::Result<Value> {
    let id_str = extract_id(args)?;
    let id_prefix = extract_id_prefix(id_str)?;
    let route = util::route_for_id_prefix(id_prefix);

    Ok(Value::String(route.to_string()))
}

fn extract_id<'a>(args: &'a HashMap<String, Value>) -> tera::Result<&'a str> {
    if let Some(Value::String(id_str)) = args.get("id") {
        Ok(id_str.as_str())
    } else if let Some(Value::Object(props)) = args.get("for") {
        if let Some(Value::String(id_str)) = props.get("id") {
            Ok(id_str.as_str())
        } else {
            Err(
                tera::Error::call_function(
                    "url",
                    UrlHelperError::InvalidArguments("value of 'for' in url(for=<model>) must be an object with an id property".to_string()),
                )
            )
        }
    } else {
        Err(
            tera::Error::call_function(
                "url",
                UrlHelperError::InvalidArguments("expected one of for=<model object> or id=<string>, got neither".to_string())
            )
        )
    }
}

fn extract_id_prefix<'a>(id_str: &'a str) -> tera::Result<&'a str> {
    match id_str.split("_").collect::<Vec<_>>().as_slice() {
        &[prefix, _token] => Ok(prefix),
        _ => Err(
            tera::Error::call_function(
                "url",
                UrlHelperError::InvalidIdentifier(id_str.to_string()),
            )
        ),
    }
}

// Look up the "action" arg, defaulting to `"view"` if it's not present
// This allows e.g., url(for=show) and url(for=show,action="view") to behave
// identically. Why do we want this? Well, when looping over a list of actions
// tera doesn't make it easy to skip passing an argument for one of those
// action names. On other other hand, we don't want to have to write out
// `action="view"` normally, so we adopt the convention that:
//
//   (missing or malformed) means the same as "view", which produces
//   a URL with no final component
fn extract_action<'a>(args: &'a HashMap<String, Value>) -> &'a str {
    if let Some(Value::String(action_str)) = args.get("action") {
        action_str.as_str()
    } else {
        "view"
    }
}

#[derive(Debug)]
enum UrlHelperError {
    InvalidArguments(String),
    InvalidIdentifier(String)
}

impl Display for UrlHelperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlHelperError::InvalidArguments(details) =>
                write!(f, "invalid arguments to url() helper function: {details}"),
            UrlHelperError::InvalidIdentifier(bad_id) =>
                write!(f, "invalid identifier {bad_id}, should be of the form \"<prefix>_<id>\""),
        }
    }
}

impl Error for UrlHelperError {}