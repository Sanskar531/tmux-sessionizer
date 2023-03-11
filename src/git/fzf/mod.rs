use std::{
    io::Write,
    process::{Command, Stdio},
};

pub struct FzfService;

impl FzfService {
    pub fn show_selection(values: Vec<String>) -> String {
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
            .as_mut()
            .unwrap()
            .write_all(values.join("\n").as_bytes())
        {
            Err(e) => panic!("{}", e),
            Ok(_) => (),
        };

        let unprocessed_branch_name = fzf_command.wait_with_output().unwrap().stdout;

        let branch_name = String::from_utf8(unprocessed_branch_name).unwrap();

        return branch_name.trim().to_string();
    }
}
