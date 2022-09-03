use std::collections::HashMap;

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

        params_map.insert(key.to_string(), value.to_string());

    }
    params_map
}

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
        assert!(boxed_get.is_some());

        let actual_param_value = boxed_get.unwrap();
        assert_eq!(actual_param_value, "empty_key");

        let boxed_get = parsed_search_params.get("empty_value");
        assert!(boxed_get.is_some());

        let actual_param_value = boxed_get.unwrap();
        assert_eq!(actual_param_value, "");
    }

    #[test]
    fn parse_empty_url_search_params() {
        let search_params = "";
        let params = parse_url_search_params(search_params);
        assert_eq!(0, params.len());
    }
}
