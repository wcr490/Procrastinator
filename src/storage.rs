#![allow(unused)]

use std::env;
use std::fmt::format;
use crate::model::*;
use crate::model;
use std::fs::{File, OpenOptions, read};
use std::path::Path;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::string::ToString;
use chrono::Local;

const CSV_FILE_NAME: &str = "log.csv";

pub fn add_item(id: usize,
                name: String,
                is_completed: bool,
                is_deleted: bool,
                created_time: String,
                completed_time: String,
                deleted_time: String) {
    let content: String = format!("{},{},{},{},{},{},{}\r\n",id,name,is_completed,is_deleted,created_time,completed_time,deleted_time);
    Csv::new()?.file.write_all(content.as_ref());
}

pub fn get_all() {

}

struct Csv {
    filename: String,
    file: File,
}

impl Csv{
    fn new() -> Result<Self,Error>{
        let name = Csv::get_filename()?;
        let path = Path::new(&name)?;
        let mut file = Csv::create(&path)?;
        if !path.exists(){
            //init logic
            file.write_all(b"id,name,is_completed,is_deleted,created_time,completed_time,deleted_time\r\n")?;
        }
        Ok(Csv{
            filename: name,
            file,
        })

    }
    fn create(path: &Path) -> Result<File, Error>{
        let csv_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&path)?;
        Ok(csv_file)
    }
    fn get_filename() -> Result<String, Error>{
        let home = env::var("HOME").map_err(|e| format!("Failed to read HOME variable: {}", e))?;
        let filename = format!("{}/{}", home, CSV_FILE_NAME);
        Ok(filename)
    }

    fn read() -> Result<Vec<String>, Error> {
        let reader = BufReader::new(Self.file);
        let mut lines = Vec::new();
        for line in reader.lines()? {
            lines.push(line);
        }
        Ok(lines)
    }
}



