use std::{env::current_dir, process::Command};

pub struct SymLink;

impl SymLink {
    pub fn link_folder(folder_name: &str, branch: &str) {
        SymLink::get_sym_link_command()
            .arg("-s")
            .arg("-t")
            .arg(format!("/home/sanskar/git-worktrees/{}/", branch))
            .arg(folder_name)
            .arg(format!(
                "{}/{}/",
                current_dir().unwrap().to_str().unwrap(),
                folder_name
            ))
            .output()
            .unwrap();
    }

    fn get_sym_link_command() -> Command {
        Command::new("ln")
    }
}
