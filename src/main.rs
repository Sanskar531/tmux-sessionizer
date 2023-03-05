use std::process::Command;

#[derive(Clone)]
struct Branch<'a> {
    name: &'a str,
}

impl<'a> Branch<'a> {
    fn clean(&mut self) {
        self.name = &self.name[2..];
    }
}

fn check_installed_commands() {
    Command::new("git").output().expect("Please install git");
    Command::new("tmux").output().expect("Please install tmux");
}

fn main() {
    check_installed_commands();
    let process_output =
        String::from_utf8(Command::new("git").arg("branch").output().unwrap().stdout).unwrap();
    let branches: Vec<Branch> = process_output
        .split("\n")
        .filter(|branch| branch.len() > 2)
        .map(|branch| -> Branch {
            let mut branch = Branch { name: branch };
            branch.clean();
            branch
        })
        .collect();
    branches.iter().for_each(|x| println!("{}", x.name));
}
