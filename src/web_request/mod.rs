use chrono::NaiveDate;
use data_structure::CsrfDoc;
use reqwest::{header::ACCEPT, Client, StatusCode};

pub mod data_structure;

pub async fn fetch_patch_tuesday_report(patch_tuesday_date: NaiveDate) {
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
                Ok(parsed) => println!("Parsed {:#?}!", parsed.vulnerability),
                Err(e) => println!(
                    "Ran into an error parsing the Patch Tuesday report (go complain to Euan - and be sure to send the error & args!):- {}",
                    e
                ),
            }
        }
        _ => panic!(
            "We seem to have run into an error with the web request. Is the connection okay?"
        ),
    }

    todo!("Format web information into a Patch Tuesday report/Teams announcement");
}
