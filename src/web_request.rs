use std::collections::HashMap;

use chrono::NaiveDate;
use reqwest::{header::ACCEPT, Client, StatusCode};
use serde::Deserialize;
use serde_json::Value;

// Note: I'm not sure if every node in the JSON file needs to be parsed, will experiment once a working prototype exists
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CsrfDoc {
    pub product_tree: ProductTree,
    pub vulnerability: Vec<Vulnerability>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProductTree {
    pub branch: Vec<Branch>,
    pub full_product_name: Vec<FullProductName>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Branch {
    pub items: Vec<Branch>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FullProductName {
    #[serde(rename = "ProductID")]
    pub product_id: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Vulnerability {
    pub title: StringVal,
    pub notes: Vec<Note>,
    pub discovery_date_specified: bool,
    pub release_date_specified: bool,
    #[serde(rename = "CVE")]
    pub cve: String,
    pub product_statuses: Vec<ProductStatus>,
    pub threats: Vec<Threat>,
    #[serde(rename = "CVSSScoreSets")]
    pub cvssscore_sets: Vec<CvssscoreSet>,
    pub remediations: Vec<Remediation>,
    pub acknowledgments: Vec<Acknowledgment>,
    pub ordinal: String,
    pub revision_history: Vec<Revision>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StringVal {
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Note {
    pub title: String,
    #[serde(rename = "Type")]
    pub type_field: i64,
    pub ordinal: String,
    pub value: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProductStatus {
    #[serde(rename = "ProductID")]
    pub product_id: Vec<String>,
    #[serde(rename = "Type")]
    pub type_field: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Threat {
    pub description: OptString,
    #[serde(rename = "ProductID")]
    #[serde(default)]
    pub product_id: Vec<String>,
    #[serde(rename = "Type")]
    pub type_field: i64,
    pub date_specified: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct OptString {
    pub value: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CvssscoreSet {
    pub base_score: f64,
    pub temporal_score: f64,
    pub vector: String,
    #[serde(rename = "ProductID")]
    pub product_id: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Remediation {
    pub description: StringVal,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    pub supercedence: Option<String>,
    #[serde(rename = "ProductID")]
    #[serde(default)]
    pub product_id: Vec<String>,
    #[serde(rename = "Type")]
    pub type_field: i64,
    pub date_specified: bool,
    pub affected_files: Vec<Value>,
    pub restart_required: Option<StringVal>,
    pub sub_type: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Acknowledgment {
    pub name: Vec<OptString>,
    #[serde(rename = "URL")]
    pub url: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Revision {
    pub number: String,
    pub date: String,
    pub description: StringVal,
}

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

    todo!(
        "Use reqwest to get the patch Tuesday report from Microsoft's RESTful API\n\
        You can access the latest report from https://api.msrc.microsoft.com/cvrf/v2.0/cvrf/{}",
        patch_tuesday_date.format("%Y-%b")
    );
}
