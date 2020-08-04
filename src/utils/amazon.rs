extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

//Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_amazon_url(query: &str) -> String {
    if query == "am" {
        let amazon_dotcom = "https://amazon.com";

        amazon_dotcom.to_string()

    }else {
        //Assume the other match is "am sometext"
        //and search on Amazon
        construct_amazon_search_url(&query[3..])
    }
}

pub fn construct_amazon_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let amazon_search_url = format!("https://amazon.com/s?k={}",
        encoded_query);
    
        amazon_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_amazon_url() {
        let fake_query = "am";
        assert_eq!(
            construct_amazon_url(fake_query),
            "https://amazon.com"
        );
    }

    #[test]
    fn test_construct_amazon_search_url() {
        let fake_query = "xbox one";
        assert_eq!(
            construct_amazon_search_url(fake_query),
            "https://amazon.com/s?k=xbox%20one"
        );
    }
}