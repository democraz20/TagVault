use std::io::{self, Write};

mod store;
mod layer;

#[cfg(target_os = "windows")]
const OS_STORE_PATH: &str = ".";

#[cfg(target_os = "linux")]
const OS_STORE_PATH: &str = ".";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    //cmdline mode
    println!("No args passed, running in cmdline mode");
    loop {
        let mut cmdin = String::new();
        print!(">");
        io::stdin().read_line(&mut cmdin)?;
        cmdin = cmdin.trim().to_string();
        dbg!(&cmdin);
        if cmdin.contains("exit") {
            break;
        }
        io::stdout().flush()?;
    }
    Ok(())
}

fn store() {}
fn get() {}
fn update() {}
fn del() {}
//display and query seperated somehow