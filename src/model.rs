#![allow(unused)]


use core::fmt;
use std::error::Error;
use chrono::format::parse;

#[derive(Debug)]
struct ItemError;

impl fmt::Display for ItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom error: Something went wrong.")
    }
}



#[derive(Debug, Clone)]
pub struct Item {
    pub(crate) id: usize,
    pub(crate)name: String,
    pub(crate) is_completed: bool,
    pub(crate) is_deleted: bool,
    pub(crate) created_time: Option<i64>,
    pub(crate) completed_time: Option<i64>,
    pub(crate) deleted_time: Option<i64>,

}

impl Item {
    pub fn new(id: usize,
               name: String,
               is_completed: bool,
               is_deleted: bool,
               created_time: Option<i64>,
               completed_time: Option<i64>,
               deleted_time: Option<i64>,

    ) -> Self {

        Item {
            id,
            name,
            is_completed,
            is_deleted,
            created_time,
            completed_time,
            deleted_time,
        }
    }
    pub fn new_by_id(searching_id: usize, lines: Vec<String>) -> Result<Item, Box<dyn Error>> {
        for line in lines{
            if let Some(index) = line.find(',') {
                let id = &line[..index];
                if id == searching_id.to_string(){
                    item_info_parse(&line)
                }
            }
        }
        Err(Box::new(ItemError))
    }

    pub fn get_id() -> usize{
        Self.id
    }

    pub fn print_formal() -> String{
        let item_info: String = format!("📇{} {} {}",Self.id, Self.name, if Self.is_completed {"☑️"}else{"⬜️"});
        let created_info: String = match Self.created_time{
            None => "Information gap".to_string(),
            Some(_) => format!(
                "{}.{}.{} {}:{}",
                Self.created_time[0..4],
                Self.created_time[4..6],
                Self.created_time[6..8],
                Self.created_time[8..9],
                Self.created_time[9..10]
            ).to_string(),
        };
        let completed_info: String = if Self.is_completed {
            match Self.completed_time {
                None => "Information gap".to_string(),
                Some(_) => format!(
                    "{}.{}.{} {}:{}",
                    Self.completed_time[0..4],
                    Self.completed_time[4..6],
                    Self.completed_time[6..8],
                    Self.completed_time[8..9],
                    Self.completed_time[9..10]
                ).to_string(),
            }
        }
            else {
                "uncompleted"
            };
        let deleted_info: String = if Self.is_deleted {
            match Self.deleted_time {
                None => "Information gap".to_string(),
                Some(_) => format!(
                    "{}.{}.{} {}:{}",
                    Self.deleted_time[0..4],
                    Self.deleted_time[4..6],
                    Self.deleted_time[6..8],
                    Self.deleted_time[8..9],
                    Self.deleted_time[9..10]
                ).to_string(),
            }
        }
            else {
                ""
            };

        format!("{}\r\ncreated at{}\r\ndeleted at{}",item_info,created_info,deleted_info)
    }
}

fn item_info_parse(line: &str) -> Result<Item, Box<dyn Error>> {
    let mut temp: (usize, String, bool, bool, Option<i64>, Option<i64>, Option<i64>) =
        (0, "".to_string(), false, false, None, None, None);
    let mut temp_counter: usize = 0;

    for content in line.split(',') {
        match temp_counter {
            0 => temp.0 = content.parse().map_err(|e| Box::new(ItemError) as Box<dyn Error>)?,
            1 => temp.1 = content.to_string(),
            2 => temp.2 = content.parse().map_err(|e| Box::new(ItemError) as Box<dyn Error>)?,
            3 => temp.3 = content.parse().map_err(|e| Box::new(ItemError) as Box<dyn Error>)?,
            4 => temp.4 = Some(content.parse().map_err(|e| Box::new(ItemError) as Box<dyn Error>)?),
            5 => temp.5 = Some(content.parse().map_err(|e| Box::new(ItemError) as Box<dyn Error>)?),
            6 => temp.6 = Some(content.parse().map_err(|e| Box::new(ItemError) as Box<dyn Error>)?),
            _ => {
                // Handle unexpected content or other error handling logic
                return Err(Box::new(ItemError));
            }
        }

        temp_counter += 1;
    }

    if temp_counter == 7 {
        Ok(Item {
            id: temp.0,
            name: temp.1,
            is_completed: temp.2,
            is_deleted: temp.3,
            created_time: temp.4,
            completed_time: temp.5,
            deleted_time: temp.6,
        })
    } else {
        println!("error");
        Err(Box::new(ItemError))
    }
}




