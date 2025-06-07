use dirs::data_local_dir;
use std::fs;
use std::path::PathBuf;

pub fn get_data_path() -> PathBuf {
    let mut path = data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("unsafe_todo");
    fs::create_dir_all(&path).ok();
    path.push("todos.txt");
    path
}
