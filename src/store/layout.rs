use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct vault {
    vaultname: String,
    vaultdesc: String,

    hash_locations: Vec<String>,
    tag_list: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct vaultlist {
    pub vaults: Vec<String> //encrypted
}