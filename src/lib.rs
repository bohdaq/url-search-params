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
            params_map.insert(decode_uri_component(key), decode_uri_component(value));
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
/// params_map.insert("key1&".to_string(), "test1=".to_string());
/// params_map.insert("key2".to_string(), "test2".to_string());
///
/// let search_params : String = build_url_search_params(params_map);
///
/// // validating output
/// let parsed_search_params: HashMap<String, String> = parse_url_search_params(&search_params);
///
/// let boxed_get = parsed_search_params.get("key1&");
/// assert!(boxed_get.is_some());
///
/// let actual_param_value = boxed_get.unwrap();
/// assert_eq!(actual_param_value, "test1=");
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
        let param = [encode_uri_component(key.as_str()), "=".to_string(), encode_uri_component(value.as_str())].join("");
        key_value_list.push(param);
    }

    key_value_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    let url_search_params : String = key_value_list.join("&");

    url_search_params
}

pub fn encode_uri_component(component: &str) -> String {
    let mut _result = component.replace(SYMBOL.percent, "%25");
    _result = _result.replace(SYMBOL.whitespace, "%20");
    _result = _result.replace(SYMBOL.carriage_return, "%0D");
    _result = _result.replace(SYMBOL.new_line, "%0A");
    _result = _result.replace(SYMBOL.exclamation_mark, "%21");
    _result = _result.replace(SYMBOL.quotation_mark, "%22");
    _result = _result.replace(SYMBOL.number_sign, "%23");
    _result = _result.replace(SYMBOL.dollar, "%24");
    _result = _result.replace(SYMBOL.ampersand, "%26");
    _result = _result.replace(SYMBOL.single_quote, "%27");
    _result = _result.replace(SYMBOL.opening_bracket, "%28");
    _result = _result.replace(SYMBOL.closing_bracket, "%29");
    _result = _result.replace(SYMBOL.asterisk, "%2A");
    _result = _result.replace(SYMBOL.plus, "%2B");
    _result = _result.replace(SYMBOL.comma, "%2C");
    _result = _result.replace(SYMBOL.slash, "%2F");
    _result = _result.replace(SYMBOL.colon, "%3A");
    _result = _result.replace(SYMBOL.semicolon, "%3B");
    _result = _result.replace(SYMBOL.equals, "%3D");
    _result = _result.replace(SYMBOL.at, "%40");
    _result = _result.replace(SYMBOL.opening_square_bracket, "%5B");
    _result = _result.replace(SYMBOL.closing_square_bracket, "%5D");


    return _result
}

pub fn decode_uri_component(component: &str) -> String {
    let mut _result = component.replace( "%20", SYMBOL.whitespace);
    _result = _result.replace("%0A", SYMBOL.new_line);
    _result = _result.replace ("%0D", SYMBOL.carriage_return);
    _result = _result.replace ("%21", SYMBOL.exclamation_mark);
    _result = _result.replace ("%22", SYMBOL.quotation_mark);
    _result = _result.replace ("%23", SYMBOL.number_sign);
    _result = _result.replace ("%24", SYMBOL.dollar);
    _result = _result.replace ("%25", SYMBOL.percent);
    _result = _result.replace ("%26", SYMBOL.ampersand);
    _result = _result.replace ("%27", SYMBOL.single_quote);
    _result = _result.replace ("%28", SYMBOL.opening_bracket);
    _result = _result.replace ("%29", SYMBOL.closing_bracket);
    _result = _result.replace ("%2A", SYMBOL.asterisk);
    _result = _result.replace ("%2B", SYMBOL.plus);
    _result = _result.replace ("%2C", SYMBOL.comma);
    _result = _result.replace ("%2F", SYMBOL.slash);
    _result = _result.replace ("%3A", SYMBOL.colon);
    _result = _result.replace ("%3B", SYMBOL.semicolon);
    _result = _result.replace ("%3D", SYMBOL.equals);
    _result = _result.replace ("%3F", SYMBOL.question_mark);
    _result = _result.replace ("%40", SYMBOL.at);
    _result = _result.replace ("%5B", SYMBOL.opening_square_bracket);
    _result = _result.replace ("%5D", SYMBOL.closing_square_bracket);

    return _result
}

pub struct Symbol {
    pub new_line_carriage_return: &'static str,
    pub new_line: &'static str,
    pub carriage_return: &'static str,
    pub empty_string: &'static str,
    pub whitespace: &'static str,
    pub equals: &'static str,
    pub comma: &'static str,
    pub hyphen: &'static str,
    pub slash: &'static str,
    pub semicolon: &'static str,
    pub colon: &'static str,
    pub number_sign: &'static str,
    pub opening_square_bracket: &'static str,
    pub closing_square_bracket: &'static str,
    pub opening_curly_bracket: &'static str,
    pub closing_curly_bracket: &'static str,
    pub quotation_mark: &'static str,
    pub underscore: &'static str,
    pub single_quote: &'static str,
    pub percent: &'static str,
    pub exclamation_mark: &'static str,
    pub dollar: &'static str,
    pub ampersand: &'static str,
    pub opening_bracket: &'static str,
    pub closing_bracket: &'static str,
    pub asterisk: &'static str,
    pub plus: &'static str,
    pub question_mark: &'static str,
    pub at: &'static str,
}

pub const SYMBOL: Symbol = Symbol {
    new_line: "\n",
    carriage_return: "\r",
    new_line_carriage_return: "\r\n",
    empty_string: "",
    whitespace: " ",
    equals: "=",
    comma: ",",
    hyphen: "-",
    slash: "/",
    semicolon: ";",
    colon: ":",
    number_sign: "#",
    opening_square_bracket: "[",
    closing_square_bracket: "]",
    opening_curly_bracket: "{",
    closing_curly_bracket: "}",
    quotation_mark: "\"",
    underscore: "_",
    single_quote: "'",
    percent: "%",
    exclamation_mark: "!",
    dollar: "$",
    ampersand: "&",
    opening_bracket: "(",
    closing_bracket: ")",
    asterisk: "*",
    plus: "+",
    question_mark: "?",
    at: "@",
};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{build_url_search_params, decode_uri_component, encode_uri_component, parse_url_search_params};

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

    #[test]
    fn encode_decode() {
        let component = "\r\n \"%!#$&'()*+,/:;=?@[]][@?=;:/,+*)('&$#!%\" \r\n";
        let mut _result = encode_uri_component(component);
        assert_eq!("%0D%0A%20%22%25%21%23%24%26%27%28%29%2A%2B%2C%2F%3A%3B%3D?%40%5B%5D%5D%5B%40?%3D%3B%3A%2F%2C%2B%2A%29%28%27%26%24%23%21%25%22%20%0D%0A", _result);
        _result = decode_uri_component(_result.as_str());
        assert_eq!(component, _result);
    }
}
