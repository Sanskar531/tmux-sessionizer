mod git;
mod tmux;

fn main() {
    use git::GitService;

    let mut git_service = GitService { branch_name: None };
    let branch = git_service.get_branch().as_ref().unwrap();
    println!("{}", branch);
}
