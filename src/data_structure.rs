
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CsrfDoc {
    document_title: String,
    document_type: String,
    document_publisher: DocumentPublisher,
    document_tracking: DocumentTracking,
    document_notes: Vec<Note>,
    product_tree: ProductTree,
    vulnerability: Vec<Vulnerability>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DocumentPublisher {
    contact_details: String,
    issuing_authority: String,
    #[serde(rename = "type")]
    publisher_type: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DocumentTracking {
    identification: Identification,
    status: u16,
    version: f32,
    revision_history: Vec<Revision>,
    initial_release_date: String,
    current_release_date: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Identification {
    id: String,
    alias: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Revision {
    number: f32,
    date: String,
    description: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Note {
    title: String,
    audience: String,
    #[serde(rename = "Type")]
    note_type: u16,
    value: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ProductTree {
    branch: Vec<Branch>,
    full_product_name: HashMap<u32, String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Branch {
    items: Vec<Items>,
    #[serde(rename = "Type")]
    branch_type: u16,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Items {
    items: Vec<Products>,
    #[serde(rename = "Type")]
    item_type: u16,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Products {
    product_id: u32,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Vulnerability {
    title: String,
    notes: Vec<Note>,
    discovery_date_specified: bool,
    release_date_specified: bool,
    cve: String,
    product_statuses: Vec<ProductStatus>,
    threats: Vec<Threats>,
    cvss_score_sets: Vec<CvssScoreSet>,
    remediations: Vec<Remediation>,
    acknowledgements: Vec<Acknowledgement>,
    revision_history: Vec<Revision>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ProductStatus {
    product_id: Vec<u32>,
    #[serde(rename = "Type")]
    status: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Threats {
    description: String,
    product_ids: Vec<u32>,
    #[serde(rename = "Type")]
    threat_type: u16,
    date_specified: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CvssScoreSet {
    base_score: f32,
    temporal_score: f32,
    vector: String,
    product_id: Vec<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Remediation {
    description: u32,
    url: String,
    supercedence: u32,
    product_id: u16,
    #[serde(rename = "Type")]
    remediation_type: u16,
    date_specified: bool,
    affected_files: Vec<String>,
    restart_required: bool,
    subtype: String,
}