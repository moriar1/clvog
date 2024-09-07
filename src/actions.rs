use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
    vec,
};

pub struct Config {
    pub is_verbose: bool,
    pub dir_path: PathBuf,
    pub list_path: PathBuf,
}

/// Returns vec filenames that start with four digits and length at least 11 symbols: "0001-AA.aaa
fn get_entries(dir_path: &PathBuf) -> Vec<String> {
    // TODO: add caching, if get_entries has been already called it shouldn't create vec again (See
    // clvog/misc/cache.rs as example)
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
    entries
}

fn get_records(list_path: &PathBuf) -> Vec<String> {
    let Some(records) = fs::read_to_string(list_path).ok() else {
        return vec![];
    };
    records.lines().map(String::from).collect::<Vec<String>>()
}

// Check if all video files and records in list have the same names, if not panic!
pub fn check(config: &Config) {
    let (dir_entries, list_records) = (
        get_entries(&config.dir_path),
        get_records(&config.list_path),
    );

    // NOTE loop check only existing records and dir entries,
    // so if you have extra records in vid_list.txt, but these videos are not exist in ./vid/
    // directory then verifing will still passes.
    // TODO: Uncomment if needs:
    // if list_records.len() != dir_entries.len() {
    //     panic!("")
    // }
    let mut i = 0; // TODO: use enumerate() from 1
    for (entry, rec) in dir_entries.iter().zip(list_records.iter()) {
        i += 1;
        let r = rec.split_whitespace().next().unwrap();

        if config.is_verbose {
            println!("Comparing. Entry:`{}` record: `{}`", entry, r);
        }
        if (*entry)[0..4].parse::<u32>().unwrap() != i {
            panic!("Broken order: `{}`\n record name: `{}`", entry, r);
        }

        if entry != r && (*entry)[5..8] != *"AAA" {
            panic!("entry name: `{}`\n record name: `{}`", entry, r);
        }
    }
}

/// 1. Get list_records and dir_entries(videos)
/// 2. Get last record number
/// 3. Write new records from <new_list_path> file with whole information (name, link, description) to
///    new_list_records vector and append <list_path> file with it.
/// 4. Or if <list_path> file do not exists and dir_entries is empty - create it, numbering starts
///    with 0001
/// 5. Rewrite <new_list_path> file with numbers
/// 6. Dowload videos using download.py (or create `AAA` dummy):
///     - it gets <new_list_path> file
///     - uses yt-dlp to download videos, renames them
///     - all error are in stderr
///     - all failed videos written in "failed_downloads.log" for further manual intervention,
///     - empty files with appropriate names are created
pub fn add(config: Config, new_list_path: PathBuf) {
    let list_records = get_entries(&config.dir_path);

    // Сreate String of new records with numbers, splited by '\n'
    let size = list_records.len() + 1;
    let new_records = fs::read_to_string(new_list_path.to_owned())
        .unwrap_or_else(|_| panic!("Error: cannot read new list: `{:?}`", new_list_path))
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{:04}-{}\n", i + size, line))
        .collect::<String>();

    // Open main video list as 'Append' if it exists, or as 'create and write' if not
    let mut list_file = if list_records.is_empty() {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&config.list_path)
            .unwrap()
    } else {
        OpenOptions::new()
            .append(true)
            .open(&config.list_path)
            .unwrap()
    };

    // Create backups of lists before writing
    // TODO: add option to skip backup
    for &list in [&new_list_path, &config.list_path].iter() {
        fs::copy(&list, list.to_str().unwrap().to_string() + ".bak")
            .unwrap_or_else(|e| panic!("Error: cannot create backup of lists: {:?}", e));
    }

    // Rewriting list with new records with numbers to then use downloader.py
    fs::write(&new_list_path, &new_records)
        .unwrap_or_else(|_| panic!("Error: cannor rewrite new list file: `{:?}`", new_list_path));

    // Appending main video list
    list_file
        .write_all(new_records.as_bytes())
        .unwrap_or_else(|e| panic!("Error: cannot write in video list: {}", e));

    // run downloader
    Command::new("python3")
        .arg("download.py")
        .arg(new_list_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        // .spawn()
        .expect("execute");
    // py.id
    // let py_stdout = String::from_utf8(py.stdout).unwrap();
    // let py_stderr = String::from_utf8(py.stderr).unwrap();
    // println!("{py_stdout}");
    // eprintln!("{py_stderr}");
    //
}
