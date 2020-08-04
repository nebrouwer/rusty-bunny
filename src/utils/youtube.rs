extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

//Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_youtube_url(query: &str) -> String {
    if query == "yt" {
        let youtube_dotcom = "https://youtube.com";

        youtube_dotcom.to_string()

    }else {
        //Assume the other match is "yt sometext"
        //and search on YouTube
        construct_youtube_search_url(&query[3..])
    }
}

pub fn construct_youtube_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let youtube_search_url = format!("https://youtube.com/search?q={}",
        encoded_query);
    
        youtube_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_youtube_url() {
        let fake_query = "yt";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://youtube.com"
        );
    }

    #[test]
    fn test_construct_youtube_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_youtube_search_url(fake_query),
            "https://youtube.com/search?q=hello%20world"
        );
    }
}