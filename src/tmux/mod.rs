use std::process::{Child, Command};

pub struct TmuxService;

impl TmuxService {
    pub fn spawn_session(branch_name: &String) -> () {
        let does_session_already_exists = TmuxService::check_tmux_sessions(branch_name);
        let mut child_process: Child;

        if does_session_already_exists {
            child_process = TmuxService::attach_created_session(branch_name);
        } else {
            child_process = TmuxService::create_new_session(branch_name);
        }

        child_process.wait();
    }

    pub fn attach_created_session(branch_name: &String) -> Child {
        match TmuxService::get_tmux_command()
            .arg("attach-session")
            .arg("-t")
            .arg(branch_name)
            .spawn()
        {
            Err(e) => panic!("{}", e),
            Ok(child_process) => child_process,
        }
    }

    pub fn create_new_session(branch_name: &String) -> Child {
        match TmuxService::get_tmux_command()
            .arg("new-session")
            .arg("-c")
            .arg(format!("/home/sanskar/git-worktrees/{}/", branch_name))
            .arg("-s")
            .arg(branch_name)
            .spawn()
        {
            Err(e) => panic!("{}", e),
            Ok(child_process) => child_process,
        }
    }

    fn get_tmux_command() -> Command {
        Command::new("tmux")
    }

    fn check_tmux_sessions(branch_name: &String) -> bool {
        let output_utf8 = Command::new("tmux").arg("ls").output().unwrap().stdout;
        let parsed_output = String::from_utf8(output_utf8).unwrap();
        let session_names = parsed_output
            .lines()
            .filter(|session| session.len() > 0)
            .map(|session| String::from(*session.split(":").collect::<Vec<&str>>().get(0).unwrap()))
            .collect::<Vec<String>>();

        session_names.contains(branch_name)
    }
}
