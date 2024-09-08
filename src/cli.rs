use clap::{arg, crate_version, Command};

#[must_use]
pub fn cli() -> Command {
    Command::new("clvog")
        .about("Command line video organizer")
        .version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("check")
                .about("Verify names of video files and records in video list are the same")
                // .arg(arg!(-u --"skip-check" "Skip names matching verifing").required(false))
                .arg(arg!(-v --"verbose" "Show debug information").required(false)),
        )
        .subcommand(
            Command::new("add")
                .about("Add new videos at the end of `vid_list.txt` from `new_vid_list.txt`")
                .arg(arg!(-d --"video-directory-path" <VIDEO_PATH> "path to the video directory, by default: `./vid/`") .required(false))
                .arg(arg!(-l --"list-path" <LIST_PATH> "path to video list to add, by default: `./new_vid_list.txt`").required(false))
                .arg(arg!(-u --"skip-check" "Skip names matching verifing").required(false))
                .arg(arg!(-v --"verbose" "Show debug information").required(false)))
        .subcommand(
            Command::new("version")
                .arg(arg!(-v --"verbose" "Show debug information").required(false)))
} //subc: help, version
