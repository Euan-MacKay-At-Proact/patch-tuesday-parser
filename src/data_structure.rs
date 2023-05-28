use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsrfDoc {
    pub document_title: ValString,
    pub document_type: ValString,
    pub document_publisher: DocumentPublisher,
    pub document_tracking: DocumentTracking,
    pub document_notes: Vec<DocumentNote>,
    pub product_tree: ProductTree,
    pub vulnerability: Vec<Vulnerability>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DocumentPublisher {
    pub contact_details: ValString,
    pub issuing_authority: ValString,
    #[serde(rename = "Type")]
    pub type_field: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DocumentTracking {
    pub identification: Identification,
    pub status: i16,
    pub version: String,
    pub revision_history: Vec<RevisionHistory>,
    pub initial_release_date: String,
    pub current_release_date: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Identification {
    #[serde(rename = "ID")]
    pub id: ValString,
    pub alias: ValString,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RevisionHistory {
    pub number: String,
    pub date: String,
    pub description: ValString,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DocumentNote {
    pub title: String,
    pub audience: Option<String>,
    #[serde(rename = "Type")]
    pub type_field: i16,
    pub ordinal: String,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProductTree {
    pub branch: Vec<Branch>,
    pub full_product_name: Vec<FullProductName>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Branch {
    pub items: Option<Vec<Branch>>,
    #[serde(rename = "Type")]
    pub type_field: Option<i64>,
    pub name: Option<String>,
    pub product_id: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FullProductName {
    #[serde(rename = "ProductID")]
    pub product_id: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Vulnerability {
    pub title: ValString,
    pub notes: Vec<Note>,
    pub discovery_date_specified: bool,
    pub release_date_specified: bool,
    #[serde(rename = "CVE")]
    pub cve: String,
    pub product_statuses: Vec<ProductStatus>,
    pub threats: Vec<Threat>,
    #[serde(rename = "CVSSScoreSets")]
    pub cvss_score_sets: Vec<CvssScoreSet>,
    pub remediations: Vec<Remediation>,
    pub acknowledgments: Vec<Acknowledgment>,
    pub ordinal: String,
    pub revision_history: Vec<RevisionHistory>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Note {
    pub title: String,
    #[serde(rename = "Type")]
    pub type_field: i16,
    pub ordinal: String,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProductStatus {
    #[serde(rename = "ProductID")]
    pub product_id: Vec<String>,
    #[serde(rename = "Type")]
    pub type_field: i16,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Threat {
    pub description: ValString,
    #[serde(rename = "ProductID")]
    #[serde(default)]
    pub product_id: Option<Vec<String>>,
    #[serde(rename = "Type")]
    pub type_field: i16,
    pub date_specified: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CvssScoreSet {
    pub base_score: f32,
    pub temporal_score: f32,
    pub vector: String,
    #[serde(rename = "ProductID")]
    pub product_id: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Remediation {
    pub description: ValString,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    pub supercedence: Option<String>,
    #[serde(rename = "ProductID")]
    pub product_id: Vec<String>,
    #[serde(rename = "Type")]
    pub type_field: i16,
    pub date_specified: bool,
    pub affected_files: Vec<AffectedFile>,
    pub restart_required: Option<ValString>,
    pub sub_type: Option<String>,
    pub fixed_build: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AffectedFile {
    pub file_name: String,
    pub file_last_modified: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Acknowledgment {
    pub name: Vec<ValString>,
    #[serde(rename = "URL")]
    pub url: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ValString {
    pub value: Option<String>,
}
