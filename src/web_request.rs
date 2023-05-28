use crate::data_structure::CsrfDoc;
use chrono::NaiveDate;
use reqwest::{header::ACCEPT, Client, StatusCode};

pub async fn fetch_patch_tuesday_report(patch_tuesday_date: NaiveDate) -> Option<CsrfDoc> {
    let url = format!(
        "https://api.msrc.microsoft.com/cvrf/v2.0/cvrf/{}",
        patch_tuesday_date.format("%Y-%b")
    );

    let client = Client::new();
    let response = client
        .get(url)
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => {
            // Now that we've verified we have a response, we need to the JSON file
            match response.json::<CsrfDoc>().await {
                Ok(parsed) => return Some(parsed),
                Err(e) => {
                    println!(
                    "Ran into an error parsing the Patch Tuesday report (go complain to Euan - and be sure to send the error & args!):- {}",
                    e
                );
                    return None;
                }
            }
        }
        _ => panic!(
            "We seem to have run into an error with the web request. Is the connection okay?"
        ),
    }
}
