//! # url search params
//!
//! `url-search-params` provides the ability to create search params from HashMap and vice versa.
//!
//! In [url](https://en.wikipedia.org/wiki/URL) (web address) search params correspond to [query string](https://en.wikipedia.org/wiki/Query_string).
//!
//! Keep in mind it works with the query string part of the URL, it is not intended to work on the whole URL by design.
//! As per specification, the question mark `?` URL delimiter is not part of a query string.
//!
//! Also hash mark `#` url delimiter and fragment part of URL is not the parts of a query string.
//! In practice, it means, the fragment and preceding hash mark won't be sent in a request to a server.
//!
use std::collections::HashMap;


/// Convert given string into a HashMap containing query string parameters as
/// key-value pairs
///
/// # Examples
///
/// ```
///    use std::collections::HashMap;
///    use url_search_params::parse_url_search_params;
///
///    let search_params: &str = "key=value&another_key=its_value";
///    let params: HashMap<String, String> = parse_url_search_params(search_params);
///
///    // validating output
///    assert_eq!(2, params.len());
///
///    let boxed_get = params.get("key");
///    assert!(boxed_get.is_some());
///
///    let actual_param_value = boxed_get.unwrap();
///    assert_eq!(actual_param_value, "value");
///
///    let boxed_get = params.get("another_key");
///    assert!(boxed_get.is_some());
///
///    let actual_param_value = boxed_get.unwrap();
///    assert_eq!(actual_param_value, "its_value");
/// ```
pub fn parse_url_search_params(params: &str) -> HashMap<String, String> {
    let mut params_map : HashMap<String, String> = HashMap::new();

    if params.trim().is_empty() {
        return params_map
    }

    let split_iter = params.split("&").into_iter();
    for param in split_iter {
        let mut key = "";
        let mut value = "";

        let mut key_value = param.split("=").into_iter();
        let boxed_key = key_value.next();
        if boxed_key.is_some() {
            key = boxed_key.unwrap();
        }

        let boxed_value = key_value.next();
        if boxed_value.is_some() {
            value = boxed_value.unwrap();
        }

        if !key.is_empty() {
            params_map.insert(key.to_string(), value.to_string());
        }

    }
    params_map
}


/// Convert given HashMap into a query string
///
/// # Examples
///
/// ```
///
/// use std::collections::HashMap;
/// use url_search_params::{build_url_search_params, parse_url_search_params};
///
/// let mut params_map: HashMap<String, String> = HashMap::new();
/// params_map.insert("key1".to_string(), "test1".to_string());
/// params_map.insert("key2".to_string(), "test2".to_string());
///
/// let search_params : String = build_url_search_params(params_map);
///
/// // validating output
/// let parsed_search_params: HashMap<String, String> = parse_url_search_params(&search_params);
///
/// let boxed_get = parsed_search_params.get("key1");
/// assert!(boxed_get.is_some());
///
/// let actual_param_value = boxed_get.unwrap();
/// assert_eq!(actual_param_value, "test1");
///
/// let boxed_get = parsed_search_params.get("key2");
/// assert!(boxed_get.is_some());
///
/// let actual_param_value = boxed_get.unwrap();
/// assert_eq!(actual_param_value, "test2");
///
///
/// ```
pub fn build_url_search_params(params: HashMap<String, String>) -> String {

    let mut key_value_list : Vec<String> = vec![];
    for (key, value) in params {
        let param = [key, "=".to_string(), value].join("");
        key_value_list.push(param);
    }

    let url_search_params : String = key_value_list.join("&");

    url_search_params
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{build_url_search_params, parse_url_search_params};

    #[test]
    fn build_url_search_params_test() {
        let mut params_map: HashMap<String, String> = HashMap::new();
        params_map.insert("key1".to_string(), "test1".to_string());
        params_map.insert("key2".to_string(), "test2".to_string());
        params_map.insert("".to_string(), "empty_key".to_string());
        params_map.insert("empty_value".to_string(), "".to_string());

        let search_params = build_url_search_params(params_map);
        let parsed_search_params = parse_url_search_params(&search_params);

        let boxed_get = parsed_search_params.get("key1");
        assert!(boxed_get.is_some());

        let actual_param_value = boxed_get.unwrap();
        assert_eq!(actual_param_value, "test1");

        let boxed_get = parsed_search_params.get("key2");
        assert!(boxed_get.is_some());

        let actual_param_value = boxed_get.unwrap();
        assert_eq!(actual_param_value, "test2");

        let boxed_get = parsed_search_params.get("");
        assert!(boxed_get.is_none());

        let boxed_get = parsed_search_params.get("empty_value");
        assert!(boxed_get.is_some());

        let actual_param_value = boxed_get.unwrap();
        assert_eq!(actual_param_value, "");
    }

    #[test]
    fn build_url_search_params_ampersand() {
        let mut params_map: HashMap<String, String> = HashMap::new();
        params_map.insert("key1".to_string(), "test1".to_string());
        params_map.insert("key2".to_string(), "test2".to_string());
        params_map.insert("".to_string(), "empty_key".to_string());
        params_map.insert("empty_value".to_string(), "".to_string());
        params_map.insert("&".to_string(), "unescaped_ampersand_as_key".to_string());

        let search_params = build_url_search_params(params_map);
        let parsed_search_params = parse_url_search_params(&search_params);

        let boxed_get = parsed_search_params.get("key1");
        assert!(boxed_get.is_some());

        let actual_param_value = boxed_get.unwrap();
        assert_eq!(actual_param_value, "test1");

        let boxed_get = parsed_search_params.get("key2");
        assert!(boxed_get.is_some());

        let actual_param_value = boxed_get.unwrap();
        assert_eq!(actual_param_value, "test2");

        let boxed_get = parsed_search_params.get("");
        assert!(boxed_get.is_none());


        let boxed_get = parsed_search_params.get("empty_value");
        assert!(boxed_get.is_some());

        let actual_param_value = boxed_get.unwrap();
        assert_eq!(actual_param_value, "");

        let boxed_get = parsed_search_params.get("empty_value");
        assert!(boxed_get.is_some());

    }

    #[test]
    fn parse_empty_url_search_params() {
        let search_params = "";
        let params = parse_url_search_params(search_params);
        assert_eq!(0, params.len());
    }

    #[test]
    fn parse_empty_equals_ampersand_search_params() {
        let search_params = "=&key2=value2";
        let params = parse_url_search_params(search_params);
        assert_eq!(1, params.len());

        let boxed_get = params.get("key2");
        assert!(boxed_get.is_some());

        let actual_param_value = boxed_get.unwrap();
        assert_eq!(actual_param_value, "value2");
    }
}
