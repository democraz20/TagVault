use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct savefile {
    //space for more metadata
    tags: Vec<tag>
}

#[derive(Serialize, Deserialize, Debug)]
struct tag {
    tagname: String, 
    filelist: Vec<String>
}

pub fn get_files_from_tags(tags: Vec<String>t) -> Vec<String> {

}