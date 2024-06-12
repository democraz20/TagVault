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
    contentlist: Vec<contentstored>,
}

//0 = filepath (absolute)
//1 = plaintext content
//2 = vaultcontent (??)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct contentstored { 
    contype: u8,
    constored: String
}

pub fn get_content_from_tag(tag: String, json: &savefile) -> Vec<contentstored> {
    for i in &json.tags {
        //has to be a better way for this later
        if i.tagname == tag {
            // println!("found tag: {:?}", tag);
            return i.contentlist.clone()
        }
    }
    vec![]
}

pub fn vector_intersection(vec1: &Vec<contentstored>, vec2: &Vec<contentstored>) -> Vec<contentstored> {
    // Convert vec1 to a HashSet for efficient lookups based on (contype, constored) tuple
    let set1: HashSet<(u8, String)> = HashSet::from_iter(vec1.iter().map(|item| (item.contype, item.constored.clone())));
  
    // Filter vec2 based on elements present in set1
    vec2.iter()
        .filter(|item| set1.contains(&(item.contype, item.constored.clone())))
        .cloned()
        .collect()
}

//legacy
// pub fn vector_intersection(vec1: &Vec<String>, vec2: &Vec<String>) -> Vec<String> {
//     // Convert vec1 to a HashSet for efficient lookups
//     let set1: HashSet<&String> = HashSet::from_iter(vec1.iter());
  
//     // Filter vec2 based on elements present in set1
//     vec2.iter()
//         .filter(|item| set1.contains(item))
//         .cloned()
//         .collect()
// }
