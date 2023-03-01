use std::path::{Path, PathBuf};
use rusqlite::{Connection, params, Result};

const CLASS_NAME: &str = "class.data";
const CLASS_CREATE_SQL: &str = "CREATE TABLE IF NOT EXISTS tb_load_class (serial_num INT PRIMARY KEY, class_id BIGINT, class_name_id BIGINT, class_name TEXT, class_status INT default '1')";
const CLASS_INSERT_SQL: &str = "INSERT INTO tb_load_class (serial_num, class_id, class_name_id, class_name, class_status) VALUES (?1, ?2, ?3, ?4, ?5)";
const CLASS_UPDATE_SQL: &str = "UPDATE tb_load_class set status = 0 where serial_num = ?1";

pub struct LoadClassDao {
    conn: Option<Connection>,
}

impl LoadClassDao {
    pub fn new(path: &PathBuf) -> Result<LoadClassDao> {
        let mut file = path.clone();
        file.push(CLASS_NAME);
        let conn = Connection::open(file)?;
        let _ = conn.execute(CLASS_CREATE_SQL, []);
        let dao = LoadClassDao {
            conn: Some(conn)
        };
        Ok(dao)
    }

    pub fn add_class(&mut self, serial_num: u32, class_id: u64, class_name_id: u64, class_name: String, class_status: u32) {
        let c = self.conn.as_mut().unwrap();
        let _ = c.execute(CLASS_INSERT_SQL, params![serial_num, class_id, class_name_id, class_name, class_status]);
    }

    pub fn add_all_class(&mut self, classes: Vec<(u32, u64, u64, String, u32)>) {
        let c = self.conn.as_mut().unwrap();
        let tx = c.transaction().unwrap();
        for class in classes {
            let _ = tx.execute(CLASS_INSERT_SQL, params![class.0, class.1, class.2, class.3, class.4]);
        }
        let _ = tx.commit();
    }

    // pub fn add_un_load_class(&mut self, serial_num: u32) {
    //     let c = self.conn.as_mut().unwrap();
    //     let _ = c.execute(CLASS_UPDATE_SQL, params![serial_num]);
    // }
    //
    // pub fn add_all_un_load_class(&mut self, serial_nums: Vec<u32>) {
    //     let c = self.conn.as_mut().unwrap();
    //     let tx = c.transaction().unwrap();
    //     for serial_num in serial_nums {
    //         let _ = tx.execute(CLASS_UPDATE_SQL, params![serial_num]);
    //     }
    //     let _ = tx.commit();
    // }
}
