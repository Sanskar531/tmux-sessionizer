use std::process::Command;

pub struct TmuxService;

impl TmuxService {
    pub fn spawn_session(branch_name: &String) -> () {
        let command = match TmuxService::get_tmux_command()
            .arg("new-session")
            .arg("-c")
            .arg(format!("~/git-worktrees/{}/", branch_name))
            .arg("-n")
            .arg(branch_name)
            .arg("-s")
            .arg(branch_name)
            .arg("nvim .")
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
