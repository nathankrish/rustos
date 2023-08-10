use stack_vec::StackVec;

use crate::console::{kprint, kprintln, CONSOLE};

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
        return self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// returns if the `exit` command is called.

fn parse_command(s: &str) {
    let mut arg_buf = [""; 64];
    let command = Command::parse(s, &mut arg_buf);
    match command {
        Ok(c) => {
            exec_command(&c);
        }, 
        Err(Error::Empty) => {},
        Err(Error::TooManyArgs) => {
            kprintln!("error: too many arguments");
        }
    }
}

fn exec_command(c: &Command) {
    kprintln!();
    match c.path() {
        "echo" => {
            for (i, arg) in c.args.iter().enumerate() {
                if i > 1 {
                    kprint!(" ");
                }
                if i > 0 {
                    kprint!("{}", arg);
                }
            }
            
        },
        path => {
            kprint!("unknown command: {}", path);
        }
    }
}

fn read_byte() -> u8 {
    let mut console = CONSOLE.lock();
    return console.read_byte();
}

fn write_byte(byte: u8) {
    let mut console = CONSOLE.lock();
    console.write_byte(byte);
}

fn backspace() {
    let mut console = CONSOLE.lock();
    console.write_byte(8);
    console.write_byte(b' ');
    console.write_byte(8);
}


pub fn shell(prefix: &str) -> ! {
    
    'main: loop {
        let mut buf = [0u8; 512];
        let mut vec = StackVec::new(&mut buf);
        kprint!("> ");
        'parse_line: loop {
            let next_char = read_byte();
            if next_char == 8 || next_char == 127 {
                // backspace
                match vec.pop() {
                    Some(c) => {
                        backspace();
                    },
                    None => {
                        // bell
                        write_byte(7);
                    }
                }
            } else if next_char == b'\r' || next_char == b'\n' {
                let line = core::str::from_utf8(vec.as_slice());
                match line {
                    Ok(s) => {
                        parse_command(s)
                    }
                    Err(e) => {}
                }
                // newline
                kprintln!();
                break;
            } else if next_char < 32 || next_char > 126 {
                // bell
                write_byte(7);
            } else {
                match vec.push(next_char) {
                    Ok(()) => {
                        write_byte(next_char)
                    },
                    Err(()) => {}
                }
            }
        }
    }
}
