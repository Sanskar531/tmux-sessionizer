use std::{env::current_dir, process::Command};

pub struct SymLink;

impl SymLink {
    pub fn link_folder(folder_name: &str, branch: &str) {
        let path_to_folder = format!(
            "{}/{}/",
            current_dir().unwrap().to_str().unwrap(),
            folder_name
        );
        let path_to_create_link = format!(
            "/home/sanskar/git-worktrees/{}/{}",
            branch,
            folder_name
        );
        SymLink::get_sym_link_command()
            .arg("-s")
            .arg(path_to_folder)
            .arg(path_to_create_link)
            .output()
            .unwrap();
    }

    fn get_sym_link_command() -> Command {
        Command::new("ln")
    }
}
