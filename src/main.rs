use std::fs::read_dir;

mod git;
mod symlink;
mod tmux;

fn main() {
    use git::{GitService, WorktreeService};
    use symlink::SymLink;
    use tmux::TmuxService;

    let git_service = GitService { branch_name: None };
    let mut worktree_service = WorktreeService { git_service };
    worktree_service.create_worktree();
    let branch = worktree_service.git_service.get_branch().unwrap();
    TmuxService::spawn_session(branch);
    // TODO: create symlinks for dependencies

    let mut directories = read_dir("./").unwrap();
    let does_node_modules_exist = directories.any(|dir| dir.unwrap().file_name() == "node_modules");
    if does_node_modules_exist {
        SymLink::link_folder("node_modules", branch);
    }
}
