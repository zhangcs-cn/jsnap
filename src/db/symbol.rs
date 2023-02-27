use std::collections::HashMap;
use std::path::PathBuf;
use std::ptr::addr_of;
use rusqlite::{Connection, params, Result};

const SYMBOL_NAME: &str = "symbol.d";
const SYMBOL_CREATE_SQL: &str = "CREATE TABLE IF NOT EXISTS tb_symbol (id BIGINT PRIMARY KEY, name TEXT)";
const SYMBOL_INSERT_SQL: &str = "INSERT INTO tb_symbol (id, name) VALUES (?1, ?2)";
const SYMBOL_SELECT_SQL: &str = "SELECT id, name FROM tb_symbol where id = ?1";

pub struct SymbolDao {
    conn: Option<Connection>,
}

impl SymbolDao {
    pub fn new(mut path: &PathBuf) -> Result<SymbolDao> {
        let mut file = path.clone();
        file.push(SYMBOL_NAME);
        let conn = Connection::open(file)?;
        let _ = conn.execute(SYMBOL_CREATE_SQL, []);
        let dao = SymbolDao {
            conn: Some(conn)
        };
        Ok(dao)
    }
    pub fn get_name(&mut self, id: u64) -> Result<String> {
        let c = self.conn.as_mut().unwrap();
        let mut statement = c.prepare(SYMBOL_SELECT_SQL)?;
        let name = statement.query_row(params![id], |row| {
            Ok(row.get(1)?)
        })?;
        Ok(name)
    }
    pub fn add(&mut self, id: u64, name: String) {
        let c = self.conn.as_mut().unwrap();
        let _ = c.execute(SYMBOL_INSERT_SQL, params![id, name]);
    }
    pub fn add_all(&mut self, symbols: HashMap<u64, String>) {
        if symbols.is_empty() {
            return;
        }
        let c = self.conn.as_mut().unwrap();
        let tx = c.transaction().unwrap();
        for symbol in symbols {
            let _ = tx.execute(SYMBOL_INSERT_SQL, params![symbol.0, symbol.1]);
        }
        let _ = tx.commit();
    }
}