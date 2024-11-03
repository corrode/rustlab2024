use std::{
    io,
    io::IsTerminal, // <--- bring is_terminal() into scope
    io::Write,      // <--- bring flush() into scope
    process::Command,
};

// This struct doesn't use lifetimes to keep the code simple.
// You can try to use `&str` instead of `String` to avoid unnecessary allocations. 👍
#[derive(PartialEq, Debug)]
struct Cmd {
    binary: String,
    args: Vec<String>,
}

impl Cmd {
    fn from_line(line: &str) -> Option<Self> {
        let mut parts = line.split_whitespace().map(String::from);
        parts.next().map(|binary| Cmd {
            binary,
            args: parts.collect(),
        })
    }

    fn run(self) {
        match Command::new(self.binary).args(self.args).spawn() {
            Ok(mut child) => {
                child.wait().expect("command wasn't running");
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

fn main() {
    loop {
        show_prompt();
        let line = read_line();
        if let Some(command) = Cmd::from_line(&line) {
            command.run();
        }
    }
}

/// If stdout is printed to a terminal, print a prompt.
/// Otherwise, do nothing. This allows to redirect the shell stdout
/// to a file or another process, without the prompt being printed.
fn show_prompt() {
    let mut stdout = std::io::stdout();
    if stdout.is_terminal() {
        write!(stdout, "> ").unwrap();
        // Flush stdout to ensure the prompt is displayed.
        stdout.flush().expect("can't flush stdout");
    }
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line from stdin");
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_cmd_is_parsed_from_empty_line() {
        assert_eq!(Cmd::from_line(""), None);
    }

    #[test]
    fn cmd_with_no_args_is_parsed() {
        assert_eq!(
            Cmd::from_line("ls"),
            Some(Cmd {
                binary: "ls".to_string(),
                args: vec![]
            })
        );
    }

    #[test]
    fn cmd_with_args_is_parsed() {
        assert_eq!(
            Cmd::from_line("ls -l"),
            Some(Cmd {
                binary: "ls".to_string(),
                args: vec!["-l".to_string()]
            })
        );
    }
}
