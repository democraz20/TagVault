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
    let jsonobj: backend::savefile = serde_json::from_str(&jsontext)?;

    if tags.len() == 1 {
        let files = backend::get_files_from_tag(tags[0].to_string(), &jsonobj);
        dbg!(files);
    } else if tags.len() > 1 {
        //get starting vec
        let mut leftovervec: Vec<String> = vec![];

        let mut startingfiles = backend::get_files_from_tag(tags[0].to_string(), &jsonobj);
        for i in tags.iter().skip(1) {
            let followfiles = backend::get_files_from_tag(i.to_string(), &jsonobj);
            let left = backend::vector_intersection(&startingfiles, &followfiles);
            startingfiles = left.clone();
            if left.len() == 0 {
                println!("no matches for supplied tags");
                break;
            }
        }
        println!("Result: {:?}", startingfiles);
    } else if tags.len() == 0 {
        println!("No tags supplied!");
    } else {
        println!("Fatal error: tags count cant be lower than 0");
    }

    dbg!(jsonobj);
    dbg!(tags);
    Ok(())
}
