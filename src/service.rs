#![allow(unused)]
use crate::storage::add_item as storage_add_item;
use chrono::{Utc, DateTime, Local, FixedOffset};

const BEIJING_OFFSET: Option<FixedOffset> = FixedOffset::east_opt(8 * 3600);

fn get_time_bj() -> String{
    let time_bj: DateTime<FixedOffset> = BEIJING_OFFSET.from_utc_datetime(&Utc::now().naive_utc());
    time_bj.format("%Y%m%d%H%M").to_string()
}

pub fn add_item() {
    let created_time: String = get_time_bj();
    storage_add_item();
}