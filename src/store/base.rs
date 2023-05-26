use std::path::PathBuf;
use rusqlite::{Connection, params, Result};
use derive_getters::Getters;

pub trait Object {}

pub trait DBStore {
    fn insert<T: Object>(&self, obj: &T) -> bool;
    fn update<T: Object>(&self, obj: &T) -> bool;
}

#[derive(Default, Getters)]
pub struct Class {
    serial: u64,
    id: u64,
    name_id: u64,
    name: String,
    status: u8,
}

impl Class {
    const NAME: &'static str = "class.data";
    const CREATE_SQL: &'static str = "CREATE TABLE IF NOT EXISTS tb_class (serial BIGINT PRIMARY KEY, id BIGINT, name_id BIGINT, name TEXT, status INT default '1')";
    const INSERT_SQL: &'static str = "INSERT INTO tb_class (serial, id, name_id, name, status) VALUES (?1, ?2, ?3, ?4, ?5)";
    const UPDATE_SQL: &'static str = "UPDATE tb_class set status = 0 where serial = ?1";
}

impl Object for Class {}

impl DBStore for Class {
    fn insert<T: Object>(&self, obj: &T) -> bool {
        todo!()
    }

    fn update<T: Object>(&self, obj: &T) -> bool {
        todo!()
    }
}



