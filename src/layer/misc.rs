use std::path::Path;

use crate::store::layout::*;

fn get_clear_vaultlist(filepath: &Path) -> Result<Vec<vault>, Box<dyn std::error::Error>> {
    let vaultlist = std::fs::read_to_string(filepath)?;
    let vaultlist: vaultlist = serde_json::from_str(&vaultlist)?;
    let mut vaults = Vec::new();
    for vault in vaultlist.vaults {
        let vault = std::fs::read_to_string(vault)?;
        let vault: vault = serde_json::from_str(&vault)?;
        vaults.push(vault);
    }
    Ok(vaults)

}