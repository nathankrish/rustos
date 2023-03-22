use stack_vec::StackVec;
use crate::console::{kprint, kprintln, CONSOLE};
use shim::io::Read;
use str;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// returns if the `exit` command is called.
pub fn shell(prefix: &str) -> ! {
    
    // indefinite loop
    loop {
        let mut char_buf = [0u8; 512];
        let mut line_buf = [0u8; 512];
        let mut line = StackVec::new(&mut line_buf);
        kprint!("{}", prefix);
        // process line loop
        loop {
            let mut console = CONSOLE.lock();
            let num_chars = console.read(&mut char_buf).expect("read to buf");
            let input = core::str::from_utf8(&char_buf[0..num_chars]).expect("utf8 to str");
            for c in input.chars() {
                if c == '\n' || c == '\r' {
                    // newline => execute command typed into the line
                    let string = core::str::from_utf8(line.as_slice()).expect("");
                    let mut parse_buf: [&str; 64] = [""; 64];
                    let command = Command::parse(&string, &mut parse_buf);
                    match command {
                        Ok(c) => exec(c),
                        Err(Error::TooManyArgs) => {
                            kprint!("error: too many args");
                        },
                        _ => {}
                    }
                    // start a new line
                    kprintln!();
                    break;
                                    
                } else if c == (8u8 as char) || c == (127u8 as char) { 
                    // backspace / delete
                    let line_len = line.len();
                    if line_len > 0 {
                        line.pop();
                        kprint!("{}",'\r');
                        for _i in 0..(line_len + prefix.len()) { 
                            kprint!(" ");
                        }
                        kprint!("{}",'\r');
                        kprint!("{}", prefix);
                        kprint!("{}", core::str::from_utf8(line.as_slice()).expect(""));
                    }
                } else if (c as u8) < 32u8 {
                    // invisible ascii
                    kprint!("{}", 7u8 as char);
                } else {
                    match line.push(c as u8) {
                        Ok(_) => {
                            kprint!("{}", c);
                        },
                        Err(_) => {}
                    }
                }
            }
        }
    }
}

fn exec(c: Command) {
    match c.path() {
        // print all the args
        "echo" => {
            for arg in c.args.iter() {
                kprintln!("{} ", arg);
            }
        }, 
        // empty command should do nothing
        "" => {},
        // if the command is unknown print an error
        other => {
            kprintln!("unknown command: {}", other);
        }
    }
}

