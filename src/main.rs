use std::{
    io::Write,
    process::{Command, Stdio},
};

fn main() {
    let git_command = Command::new("git")
        .arg("branch")
        .output()
        .expect("Failed to launch git command");
    let git_output = String::from_utf8(git_command.stdout).unwrap();
    let mut fzf_command = match Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(e) => panic!("{}", e),
        Ok(command) => command,
    };

    match fzf_command
        .stdin
        .take()
        .unwrap()
        .write_all(git_output.as_bytes())
    {
        Err(e) => panic!("Error: {}", e),
        _ => (),
    }

    let branch_name = String::from_utf8(fzf_command.wait_with_output().unwrap().stdout).unwrap();
    let _clean_branch_name = &branch_name[2..];
}
