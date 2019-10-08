use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct CDDService {
    pub bin_path: String,
    pub template_path: String,
    pub project_path: String,
}
