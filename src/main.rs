use std::{path::Path, fs::File, error::Error};

use chrono::offset::Utc;
fn main() -> Result<(),Box<dyn Error>> {
    println!("Hello, world!");
    let d = Utc::now();
    let sd = d.date_naive().to_string();
    println!("Date: {}", sd);



    let folder = ".";
    let file = format!("{}.log",sd);
    let path_name = format!("{}/{}",folder,file);
    let path = Path::new(path_name.as_str());
    let file_exists = path.exists();



    
    if file_exists {
        println!("File already exists");
    } else {
        File::create(path_name)?;
        println!("Created File");
    };

    Ok(())
}
