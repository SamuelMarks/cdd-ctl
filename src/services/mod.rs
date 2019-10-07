use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct CDDService {
    pub bin_path: String,
}
