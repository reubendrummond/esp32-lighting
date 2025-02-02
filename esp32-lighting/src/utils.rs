use std::collections::HashMap;

use url::form_urlencoded;

pub fn get_query_value(query: &str, key: &str) -> Option<String> {
    form_urlencoded::parse(query.as_bytes())
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.to_string())
}

pub fn url_encode_params(params: &HashMap<&str, &str>) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("&")
}
