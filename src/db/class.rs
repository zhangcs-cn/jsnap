use rusqlite::{Connection, params, Result};

const CLASS_NAME: &str = "class.data";
const CLASS_CREATE_SQL: &str = "CREATE TABLE IF NOT EXISTS tb_load_class (serial_num INT PRIMARY KEY, class_id BIGINT, name_id TEXT)";
const UN_LOAD_CLASS_CREATE_SQL: &str = "CREATE TABLE IF NOT EXISTS tb_un_load_class (serial_num INT PRIMARY KEY)";

const CLASS_INSERT_SQL: &str = "INSERT INTO tb_load_class (serial_num, class_id, class_name) VALUES (?1, ?2, ?3)";
const UN_LOAD_CLASS_INSERT_SQL: &str = "INSERT INTO tb_un_load_class (serial_num) VALUES (?1)";


pub struct LoadClassDao {
    conn: Option<Connection>,
}

impl LoadClassDao {
    pub fn new() -> Result<LoadClassDao> {
        let conn = Connection::open(CLASS_NAME)?;
        let _ = conn.execute(CLASS_CREATE_SQL, []);
        let _ = conn.execute(UN_LOAD_CLASS_CREATE_SQL, []);
        let dao = LoadClassDao {
            conn: Some(conn)
        };
        Ok(dao)
    }
    pub fn add_class(&mut self, serial_num: u32, class_id: u64, name_id: u64) {
        let c = self.conn.as_mut().unwrap();
        let _ = c.execute(CLASS_INSERT_SQL, params![serial_num, class_id, name_id]);
    }
    pub fn add_all_class(&mut self, classes: Vec<(u32, u64, u64)>) {
        let c = self.conn.as_mut().unwrap();
        let tx = c.transaction().unwrap();
        for class in classes {
            let _ = tx.execute(CLASS_INSERT_SQL, params![class.0, class.1, class.2]);
        }
        let _ = tx.commit();
    }

    pub fn add_un_load_class(&mut self, serial_num: u32) {
        let c = self.conn.as_mut().unwrap();
        let _ = c.execute(UN_LOAD_CLASS_INSERT_SQL, params![serial_num]);
    }

    pub fn add_all_un_load_class(&mut self, serial_nums: Vec<u32>) {
        let c = self.conn.as_mut().unwrap();
        let tx = c.transaction().unwrap();
        for serial_num in serial_nums {
            let _ = tx.execute(UN_LOAD_CLASS_INSERT_SQL, params![serial_num]);
        }
        let _ = tx.commit();
    }
}
