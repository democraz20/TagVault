use std::path::Path;
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};

use crate::store::layout::*;

pub fn get_clear_vaultlist(full_file: String) -> Result<Vec<vault>, Box<dyn std::error::Error>> {
    let vaultlist: vaultlist = serde_json::from_str(&full_file)?;
    let mut vaults = Vec::new();
    for vault_enc in vaultlist.vaults {
        let vault = STANDARD_NO_PAD.decode(vault_enc.as_bytes())?;
        let vault = String::from_utf8_lossy(&vault);
        let vault: vault = serde_json::from_str(&vault)?;
        vaults.push(vault);
    }
    Ok(vaults)
}

pub fn vault_search(full_file: String, target: String) -> Result<vault, Box<dyn std::error::Error>> {
    let vaultlist: vaultlist = serde_json::from_str(&full_file)?;
    for un_enc in vaultlist.vaults {
        let vault = STANDARD_NO_PAD.decode(un_enc.as_bytes())?;
        let vault = String::from_utf8_lossy(&vault);
        let vault: vault = serde_json::from_str(&vault)?;
        if vault.vaultname == target {
            return Ok(vault);
        }
    }
    Err("Vault not found".into())
}