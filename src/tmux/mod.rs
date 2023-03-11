use std::{env::current_dir, process::Command};

pub struct TmuxService;

impl TmuxService {
    pub fn spawn_session(branch_name: &String) -> () {
        TmuxService::get_tmux_command()
            .arg("new-session")
            .arg("-c")
            .arg(current_dir().unwrap())
            .arg("-n")
            .arg(branch_name)
            .arg("-s")
            .arg(branch_name)
            .output();
    }

    fn get_tmux_command() -> Command {
        Command::new("tmux")
    }
}
