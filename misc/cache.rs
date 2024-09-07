use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref ENTRIES_CACHE: Mutex<HashMap<PathBuf, Vec<String>>> = Mutex::new(HashMap::new());
    static ref RECORDS_CACHE: Mutex<HashMap<PathBuf, Vec<String>>> = Mutex::new(HashMap::new());
}

fn get_entries(dir_path: &PathBuf) -> Vec<String> {
    let mut entries_cache = ENTRIES_CACHE.lock().unwrap();
    if let Some(entries) = entries_cache.get(dir_path) {
        entries.clone()
    } else {
        let entries = fs::read_dir(dir_path)
            .expect("Error: cannot open dir")
            .map(|e| {
                e.map(|e| {
                    e.file_name()
                        .into_string()
                        .map_err(|err| println!("Error: filename: {:#?}", err))
                        .unwrap()
                })
            })
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        let mut entries = entries
            .into_iter()
            .filter(|e| e.len() > 10 && e[0..4].chars().filter(|e| e.is_numeric()).count() == 4)
            .collect::<Vec<_>>();

        entries.sort();
        entries_cache.insert(dir_path.clone(), entries.clone());
        entries
    }
}

fn get_records(list_path: &PathBuf) -> Vec<String> {
    let mut records_cache = RECORDS_CACHE.lock().unwrap();
    if let Some(records) = records_cache.get(list_path) {
        records.clone()
    } else {
        let Some(records) = fs::read_to_string(list_path).ok() else {
            return vec![];
        };
        let records = records.lines().map(String::from).collect::<Vec<String>>();
        records_cache.insert(list_path.clone(), records.clone());
        records
    }
}

// Or may implement struct that contains vectors like here
//
// pub struct Organizer {
//     dir_path: &'static str,
//     list_path: &'static str,
//     dir_entries: Vec<String>,
//     list_records: Vec<String>, // whole lines or only names
//     is_verbose: bool,
// } // impl: add, check, insert, remove, sync, pull, move

// impl Organizer {
//     pub fn build() -> Organizer;
//     pub fn add();
//     etc.
// }
