use std::path::PathBuf;

use clvog::*;

fn main() {
    let matches = clvog::cli().get_matches();
    let Some(matches) = matches.subcommand() else {
        panic!("subcommand is `None`");
    };
    // takes sub_match optional argument
    let is_verbose = matches.1.get_flag("verbose");

    let (dir_path, list_path) = (PathBuf::from("./vid"), PathBuf::from("./vid_list.txt"));
    let config = Config {
        dir_path,  // PathBuf
        list_path, // PathBuf
        is_verbose,
    };

    // verify dir-list names before proceeding
    if matches.0 != "check" && !matches.1.get_flag("skip-check") {
        check(&config);
    }

    match matches {
        ("check", _) => {
            check(&config);
            println!("Verifing passed.");
        }
        ("add", _sub_match) => {
            let new_list_path = PathBuf::from("./new_vid_list.txt");
            actions::add(config, new_list_path.to_owned());
        }
        // ("hide", sub_match) => dir_actions::DirOrganizer::build().hide(),
        _ => todo!(),
    }

    /*if let Some(("hide", sub_match)) = matches.subcommand() {
     //Dir only action trait
        hide(is_verbose);
        return;
    }
    organizer: Organizer;
    if !is_skip_check {
        check();
    }
    match matches.subcommand() {
        // Organizer
        Some(("check"))
        Some(("add", sub_match)) => add(is_verbose),
        Some(("rm", sub_match)) => remove(), // check, del from vec then rewrite
        Some(("insert", sub_match)) => insert(), // check, insert into vec then rewrite
        Some(("sync", sub_match)) => sync(), // check, rewrite vec then rewrite
    }*/
}

/*            // Or create Organizer then use add()
    let is_skip_check = sub_match.get_flag("skip");
    let is_verbose = sub_match.get_flag("verbose");
    if !is_skip_check {
        check(dir_entries, records); // panic
    }
    list_records = get_records(list_path); // if exists
    dir_entries = get_entries(dir_path)

    add(
        new_list_path,
        list_path,
        dir_path,
        dir_entries,
        list_records,
        is_verbose
    );
*/
