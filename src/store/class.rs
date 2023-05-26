use std::path::PathBuf;
use derive_getters::Getters;
use rusqlite::Connection;
use crate::store::base::{DBStore, Object};

const NAME: &'static str = "class.data";
const CREATE_SQL: &'static str = "CREATE TABLE IF NOT EXISTS tb_class (serial BIGINT PRIMARY KEY, id BIGINT, name_id BIGINT, name TEXT, status INT default '1')";
const INSERT_SQL: &'static str = "INSERT INTO tb_class (serial, id, name_id, name, status) VALUES (?1, ?2, ?3, ?4, ?5)";
const UPDATE_SQL: &'static str = "UPDATE tb_class set status = 0 where serial = ?1";

// #[derive(Default, Getters)]
// pub struct Class {
//     serial: u64,
//     id: u64,
//     name_id: u64,
//     name: String,
//     status: u8,
// }
//
// impl Class {
//
//
//
// }

