extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

//Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_facebook_url(query: &str) -> String {
    if query == "fb" {
        let facebook_dotcom = "https://facebook.com";

        facebook_dotcom.to_string()

        //Check if it looks like a Facebook profile
    }//else if &query[..4] == "tw @" {
     //   construct_facebook_profile_url(&query[4..])
    //}
    else{
        //Assume the other match is "fb sometext"
        //and search on Facebook
        construct_facebook_search_url(&query[3..])
    }
}

pub fn construct_facebook_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let facebook_search_url = format!("https://facebook.com/search?q={}",
        encoded_query);
    
        facebook_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_facebook_url() {
        let fake_query = "fb";
        assert_eq!(
            construct_facebook_url(fake_query),
            "https://facebook.com"
        );
    }

    #[test]
    fn test_construct_facebook_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_facebook_search_url(fake_query),
            "https://facebook.com/search?q=hello%20world"
        );
    }
}