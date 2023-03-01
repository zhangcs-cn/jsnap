use std::path::PathBuf;
use rusqlite::{Connection, params, Result};

const THREAD_NAME: &str = "thread.data";
const THREAD_CREATE_SQL: &str = "CREATE TABLE IF NOT EXISTS tb_thread (serial_num INT PRIMARY KEY, thread_obj_id BIGINT,trace_serial_num INT, )";
const THREAD_INSERT_SQL: &str = "INSERT INTO tb_load_class (serial_num, class_id, class_name) VALUES (?1, ?2, ?3)";

pub struct Thread {
    thread_serial_num: u32,
    thread_obj_id: u64,
    trace_serial_num: u32,
    t_name_index: u64,
    g_name_index: u64,
    p_name_index: u64,
}

pub struct ThreadDao {
    conn: Option<Connection>,
}

impl ThreadDao {
    pub fn new(path: &PathBuf) -> Result<ThreadDao> {
        let mut file = path.clone();
        file.push(THREAD_NAME);
        let conn = Connection::open(file)?;
        let _ = conn.execute(THREAD_CREATE_SQL, []);
        let dao = ThreadDao {
            conn: Some(conn)
        };
        Ok(dao)
    }
}