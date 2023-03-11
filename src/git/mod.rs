use std::process::Command;

pub mod fzf;
use fzf::FzfService;

pub struct GitService {
    pub branch_name: Option<String>,
}

impl GitService {
    pub fn get_all_branches() -> Vec<String> {
        let output = match GitService::get_git_command().arg("branch").output() {
            Err(e) => panic!("{}", e),
            Ok(output) => output,
        };

        let git_output = String::from_utf8(output.stdout).unwrap();
        return git_output
            .lines()
            .filter(|branch| branch.len() > 2)
            .map(|branch| String::from(branch))
            .map(|branch| branch[2..].to_string())
            .collect();
    }

    pub fn get_branch(&mut self) -> &Option<String> {
        match self.branch_name {
            None => {
                let branches = GitService::get_all_branches();
                let selected_branch_name = FzfService::show_selection(branches);
                self.branch_name = Some(selected_branch_name);
            }
            Some(_) => (),
        };
        return &self.branch_name;
    }

    fn get_git_command() -> Command {
        return Command::new("git");
    }
}
