mod parsers;
use serial;
use structopt;
use structopt_derive::StructOpt;
use xmodem::{Xmodem, Progress};

use std::path::PathBuf;
use std::time::Duration;
use std::fs;
use std::io::Write;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}
fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader};

    let opt = Opt::from_args();
    let mut port = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    // input -> file reading
    // baud rate, timeout, char_width, tty_path, flow_control, stop_bits, raw
    //      x       x           x         x           x            x       x 

    let mut settings = port.read_settings().unwrap();
    settings.set_baud_rate(opt.baud_rate).expect("can't set baud rate");
    settings.set_char_size(opt.char_width);
    settings.set_flow_control(opt.flow_control);
    settings.set_stop_bits(opt.stop_bits);
    
    port.write_settings(&settings).expect("can't write settings");
    port.set_timeout(Duration::from_secs(opt.timeout)).expect("can't set timeout");

    let mut buffer = Vec::<u8>::new();
    match opt.input {
        None => {
            io::copy(&mut io::stdin(), &mut buffer).expect("copy from stdin to buffer");
        },
        Some(input) => {
            let mut file = BufReader::new(File::open(input).expect("can't open file"));
            io::copy(&mut file, &mut buffer).expect("copy from file to buffer");
        }
    }
    let input = buffer.as_slice();
    if opt.raw {
        let bytes = port.write(input).expect("write to port failed");
        println!("wrote {} bytes to input", bytes);
    } else {
        let bytes = Xmodem::transmit_with_progress(input, port, progress_fn)
                            .expect("write with xmodem failed");
        println!("wrote {} bytes to input", bytes);
    }
}
