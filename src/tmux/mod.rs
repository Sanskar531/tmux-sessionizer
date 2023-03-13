use std::process::Command;

pub struct TmuxService;

impl TmuxService {
    pub fn spawn_session(branch_name: &String) -> () {
        match TmuxService::get_tmux_command()
            .arg("new-session")
            .arg("-c")
            .arg(format!("/home/sanskar/git-worktrees/{}/", branch_name))
            .arg("-s")
            .arg(branch_name)
            .arg("-d")
            .output()
        {
            Ok(cmd) => cmd,
            Err(e) => panic!("{}", e),
        };
    }

    fn get_tmux_command() -> Command {
        Command::new("tmux")
    }
}
