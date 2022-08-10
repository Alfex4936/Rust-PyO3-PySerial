use std::fs::OpenOptions;
use std::io::{self, Error, Write};
use std::time::Duration;

use clap::{Arg, Command};

fn main() -> Result<(), Error> {
    let matches = Command::new("dwm1001 serial connector")
        .author("SeokWon")
        .about("Reads data from a serial port and echoes it to stdout or log file")
        .disable_version_flag(false)
        .arg(
            Arg::new("port")
                .short('p')
                .help("The device path to a serial port")
                .use_value_delimiter(false)
                .default_value("/dev/ttyACM0")
                .required(true),
        )
        .arg(
            Arg::new("baud")
                .short('b')
                .help("The baud rate")
                .use_value_delimiter(false)
                .required(false)
                .default_value("115200")
                .validator(valid_baud),
        )
        .arg(
            Arg::new("stdout")
                .short('s')
                .help("Print console output to stdout?")
                .use_value_delimiter(false)
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("log")
                .short('l')
                .help("Save output to a file, ex) a.txt")
                .use_value_delimiter(false)
                // .default_value("log.txt")
                .required(false),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .help("Port timeout milisecond")
                .use_value_delimiter(false)
                .default_value("10")
                .required(false),
        )
        .get_matches();

    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap().parse::<u32>().unwrap();
    let time_out = matches.value_of("timeout").unwrap().parse::<u64>().unwrap();
    let mut file_name = "log.txt";

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(time_out))
        .open();

    let mut stdout = false;
    if matches.is_present("stdout") {
        println!("Print result to stdout");
        stdout = true;
    }

    let mut use_log = false;
    if matches.is_present("log") {
        file_name = match matches.value_of("log") {
            Some(f) => f,
            None => "log.txt",
        };
        println!("Saving result to {}", file_name);
        use_log = true;
    }
    let mut log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(file_name)?;

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 512];
            println!("Receiving data on {} at {} baud!", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        if stdout {
                            io::stdout().write_all(&serial_buf[..t]).unwrap();
                        }
                        if use_log {
                            log_file.write_all(&serial_buf[..t]).unwrap();
                        }

                        // let msg = std::str::from_utf8(&serial_buf[..t]).unwrap();
                        // println!("{:?}", msg);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

fn valid_baud(val: &str) -> Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}
