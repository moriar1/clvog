#[allow(dead_code)]
pub struct DirOrganizer {
    dir_path: &'static str,
    dir_entries: Vec<String>,
    is_verbose: bool,
} // impl: build, hide

impl DirOrganizer {
    pub fn build() -> DirOrganizer {
        println!("DirOrganizer::build() call");
        todo!();
        // use get_entries() from dir_actions
    }
    pub fn hide(&self) {
        println!("hide() call")
    }
}
