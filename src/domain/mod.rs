#[derive(Debug, Clone)]
pub struct SoftwareItem {
    pub name: String,
    pub package_id: String,
    pub version: String,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct UpdateItem {
    pub name: String,
    pub package_id: String,
    pub installed_version: String,
    pub available_version: String,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct UpdatePlan {
    pub package_id: String,
    pub confirmed: bool,
    pub dry_run: bool,
    pub requested_mode: String,
    pub mode: String,
    pub risk: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct SourceRecommendation {
    pub software_name: String,
    pub current_source: String,
    pub recommended_source: String,
    pub score: u8,
    pub reasons: Vec<String>,
}
