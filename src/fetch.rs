use reqwest::blocking::{Client, Response};
use snailquote::unescape;

use crate::reference::Reference;
use crate::version::BibleVersion;

const ESV_URL_PREFIX: &str = "https://api.esv.org/v3/passage/text/";

fn fetch_esv(reference: Reference) -> Result<String, &'static str> {
    let api_key: String = match std::env::var("BIBLE_ESV_API_KEY") {
        Ok(t) => t,
        Err(_e) => return Err("No API key set in environment"),
    };
    let url: &str =
        &(ESV_URL_PREFIX.to_string() + "?q=" + &reference.to_string());

    let client: Client = Client::new();

    let response: Response = match client
        .get(url)
        .header("Authorization", "Token ".to_string() + &api_key)
        .send()
    {
        Ok(t) => t,
        Err(_e) => return Err("GET request failed"),
    };

    let response_data: String = match response.text() {
        Ok(t) => t,
        Err(_e) => return Err("Failed to extract response"),
    };
    let parsed_json: serde_json::Value =
        match serde_json::from_str(&response_data) {
            Ok(t) => t,
            Err(_e) => return Err("Received non-JSON response"),
        };
    let raw_verse: String = match parsed_json.get("passages") {
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(""),
        Some(_) | None => {
            return Err("Received malformed JSON response from ESV gateway")
        }
    };

    let actual_verse: String = match unescape(&raw_verse) {
        Ok(t) => t,
        Err(_e) => return Err("Failed to unescape received passage text"),
    };

    Ok(actual_verse)
}

pub fn fetch(
    reference: Reference,
    version: BibleVersion,
) -> Result<String, &'static str> {
    match version {
        BibleVersion::EnglishStandardVersion => fetch_esv(reference),
        _ => unimplemented!(),
    }
}
