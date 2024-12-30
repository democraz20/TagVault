use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};

#[derive(Serialize, Deserialize, Debug)]
pub struct vault {
    pub vaultname: String,
    pub vaultdesc: String,

    pub hash_locations: Vec<String>,
    pub tag_list: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct vaultlist {
    pub vaults: Vec<String> //encrypted
}