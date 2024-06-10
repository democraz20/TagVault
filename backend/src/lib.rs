use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct savefile {
    //space for more metadata
    tags: Vec<tag>,
}

#[derive(Serialize, Deserialize, Debug)]
struct tag {
    tagname: String,
    filelist: Vec<String>,
}

pub fn get_files_from_tag(tag: String, json: &savefile) -> Vec<String> {
    for i in &json.tags {
        //has to be a better way for this later
        if i.tagname == tag {
            println!("found tag: {:?}", tag);
            return i.filelist.clone()
        }
    }
    vec![]
}

pub fn vector_intersection(vec1: &[String], vec2: &[String]) -> Vec<String> {
    // Convert vec1 to a HashSet for efficient lookups
    let set1: HashSet<&String> = HashSet::from_iter(vec1);

    // Filter vec2 based on elements present in set1
    vec2.iter()
        .filter(|item| set1.contains(item))
        .cloned()
        .collect()
}
