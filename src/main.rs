use frontend;
use backend;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    #[cfg(target_os = "linux")]
    let jsonpath = "/home/mozartch/tagvault.json";
    #[cfg(target_os = "windows")]
    let jsonpath = "";

    println!("TagVault");
    let tags = frontend::parse_tags_input();
    
    let jsontext = fs::read_to_string(jsonpath)?;

    let jsonobj: backend::savefile = serde_json::frm_str(&jsontext)?;

    dbg!(jsonobj);

    dbg!(tags);
    Ok(())
}
