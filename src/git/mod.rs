use std::{collections::btree_map::RangeMut, fmt, process::Command};

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

    fn get_current_branch() -> String {
        let command = GitService::get_git_command()
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output();

        String::from_utf8(command.unwrap().stdout)
            .unwrap()
            .trim()
            .to_string()
    }

    fn get_git_command() -> Command {
        return Command::new("git");
    }

    fn get_all_worktrees() -> Vec<String> {
        let command = GitService::get_git_command()
            .arg("worktree")
            .arg("list")
            .arg("--porcelain")
            .output();

        String::from_utf8(command.unwrap().stdout)
            .unwrap()
            .lines()
            .filter(|line| line.len() >= 1)
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .filter(|line| *line.get(0).unwrap() == "worktree")
            .map(|line| String::from(*line.get(1).unwrap()))
            .collect::<Vec<String>>()
    }

    pub fn create_worktree(branch: &String) -> () {
        let current_branch = GitService::get_current_branch();

        if &current_branch == branch {
            panic!("Please choose a branch that has not been checked out.")
        }

        let worktrees = GitService::get_all_worktrees();
        println!("{:#?}", worktrees);

        let does_worktree_exist = worktrees
            .into_iter()
            .any(|worktree| worktree.contains(branch));

        if does_worktree_exist {
            panic!("Worktree already exists.");
        }

        let command = match GitService::get_git_command()
            .arg("worktree")
            .arg("add")
            .arg(format!("~/git-worktrees/{}/", branch))
            .arg(branch)
            .output()
        {
            Ok(command) => command,
            Err(e) => panic!("{}", e),
        };
        println!("{}", command.status);
    }
}
