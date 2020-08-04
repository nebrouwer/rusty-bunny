extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

//Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_pinterest_url(query: &str) -> String {
    if query == "pin" {
        let pinterest_dotcom = "https://pinterest.com";

        pinterest_dotcom.to_string()

    }else {
        //Assume the other match is "pin sometext"
        //and search on Pinterest
        construct_pinterest_search_url(&query[4..])
    }
}

pub fn construct_pinterest_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let pinterest_search_url = format!("https://pinterest.com/search?q={}",
        encoded_query);
    
        pinterest_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_pinterest_url() {
        let fake_query = "pin";
        assert_eq!(
            construct_pinterest_url(fake_query),
            "https://pinterest.com"
        );
    }

    #[test]
    fn test_construct_pinterest_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_pinterest_search_url(fake_query),
            "https://pinterest.com/search?q=hello%20world"
        );
    }
}