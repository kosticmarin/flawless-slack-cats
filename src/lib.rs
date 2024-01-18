use anyhow::Result;
use flawless::{
    http::{self, HTTP},
    workflow,
};
use flawless_slack::{http_client::FlawlessHttpClient, SlackClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CatImage {
    url: String,
}

fn get_cat_image() -> Result<Vec<CatImage>> {
    let request = http::request::Request::get("https://api.thecatapi.com/v1/images/search")
        .body(())?
        .send()
        .unwrap();
    let body = request.body();
    let parsed: Vec<CatImage> = serde_json::from_slice(&body)?;
    Ok(parsed)
}

#[workflow("daily_cat")]
fn meteo_hr() {
    // Get random cat image url from the API
    let random_cat_image = get_cat_image();
    match random_cat_image {
        Ok(data) => {
            // Get Slack secret token
            let secret_token = flawless::secret::get("SLACK_SECRET_TOKEN");
            if secret_token.is_none() {
                log::error!("\"SLACK_SECRET_TOKEN\" secret is not set, use `flawless secret set <VARIABLE> <VALUE>` to set it");
            } else {
                // Forward to Slack channel
                let slack_client = SlackClient::new(
                    secret_token.unwrap(),
                    FlawlessHttpClient {},
                );
                slack_client
                    .send_message(
                        "#general",
                        format!("Here is a random cat image: {}", data[0].url).as_str(),
                    )
                    .ok();
            }
        }
        Err(e) => log::error!("Error: {e}"),
    }
}
