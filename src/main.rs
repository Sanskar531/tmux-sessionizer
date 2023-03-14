mod git;
mod tmux;

fn main() {
    use git::GitService;
    use tmux::TmuxService;

    let mut git_service = GitService { branch_name: None };
    let branch = git_service.get_branch().as_ref().unwrap();
    let worktree_created = GitService::create_worktree(branch);
    if worktree_created{
        TmuxService::spawn_session(branch);
    }
}
