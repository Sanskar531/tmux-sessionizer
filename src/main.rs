mod git;
mod tmux;

fn main() {
    use git::GitService;
    use tmux::TmuxService;

    let mut git_service = GitService { branch_name: None };
    let branch = git_service.get_branch().as_ref().unwrap();
    GitService::create_worktree(branch);
    TmuxService::spawn_session(branch);
    TmuxService::attach_session(branch);
}
