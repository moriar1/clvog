use std::path::Path;

use clap::crate_version;
use clvog::{actions, Config};

fn main() {
    let matches = clvog::cli().get_matches();
    let Some(matches) = matches.subcommand() else {
        panic!("subcommand is `None`");
    };
    // takes sub_match optional argument
    let is_verbose = matches.1.get_flag("verbose");

    let (dir_path, list_path) = (Path::new("./vid"), Path::new("./vid_list.txt"));
    let config = Config {
        is_verbose,
        dir_path,
        list_path,
    };

    match matches {
        ("check", _) => {
            actions::check(&config);
            println!("Verifing passed.");
        }
        ("add", sub_match) => {
            if !sub_match.get_flag("skip-check") {
                actions::check(&config);
            };

            let default_path = "./new_vid_list.txt".to_string();
            let new_list_path = sub_match
                .get_one::<String>("list-path")
                .unwrap_or(&default_path);
            let new_list_path = Path::new(new_list_path);

            let default_path = "./vid/".to_string();
            let vid_path = sub_match
                .get_one::<String>("video-directory-path")
                .unwrap_or(&default_path);
            let vid_path = Path::new(vid_path);

            actions::add(&config, new_list_path, vid_path);
        }
        ("version", _) => println!("clvog {}", crate_version!()),
        // ("hide", sub_match) => dir_actions::DirOrganizer::build().hide(),
        _ => todo!(),
    }
}
