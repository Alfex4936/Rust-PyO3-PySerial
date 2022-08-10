use pyo3::prelude::*;
// use pyo3::types::{PyInt, PyString};

use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::time::Duration;

/// Decawave dwm1001 serial connector (contiki-os)
#[pyclass(name = "UWB", freelist = 100)]
pub struct UWB {
    /// Port name to connect
    #[pyo3(get)]
    port_name: String,
    /// Baudrate
    #[pyo3(get)]
    baudrate: u32,
    // Serial timeout ms
    #[pyo3(get)]
    timeout: u64,
    // log file name
    #[pyo3(get)]
    log_file: String,
}

#[pymethods]
impl UWB {
    /// port, baudrate, timeout, log_file
    #[new]
    fn new(
        port: String,
        baudrate: Option<u32>,
        timeout: Option<u64>,
        log_file: Option<String>,
    ) -> Self {
        UWB {
            port_name: port.to_string(),
            baudrate: baudrate.unwrap_or(115200),
            timeout: timeout.unwrap_or(100),
            log_file: log_file.unwrap_or("None".to_string()),
        }
    }

    /// Get all available ports
    #[staticmethod]
    fn get_available_ports() -> PyResult<Vec<String>> {
        let ports = serialport::available_ports().expect("No ports found!");

        let mut all_ports = Vec::new();
        for p in ports {
            all_ports.push(format!("{} ({:?})", p.port_name, p.port_type));
        }

        Ok(all_ports)
    }

    /// Connect to port infinitely
    #[pyo3(text_signature = "($self, stdout, append, interactive, /)")]
    fn connect(
        &self,
        stdout: Option<bool>,
        append: Option<bool>,
        interactive: Option<bool>,
        py: Python<'_>,
    ) -> PyResult<()> {
        // ctrlc::set_handler(|| ::std::process::exit(1)).unwrap();

        // let port = serialport::new(&self.port_name, self.baudrate)
        //     .timeout(Duration::from_millis(self.timeout))
        //     .open();
        let port = serialport::new(&self.port_name, self.baudrate)
            .timeout(Duration::from_millis(self.timeout))
            .open_native();

        let mut log_file = if self.log_file.ne("None") {
            Some(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(append.unwrap_or(false))
                    .open(&self.log_file)
                    .unwrap(),
            )
        } else {
            None
        };

        match port {
            Ok(mut port) => {
                port.set_exclusive(false).expect("Non exclusive");
                let mut serial_buf: Vec<u8> = vec![0; 512];
                println!(
                    "Receiving data on {} at {} baud!",
                    &self.port_name, &self.baudrate
                );
                loop {
                    match py.check_signals() {
                        Err(e) => {
                            std::mem::drop(&port);
                            println!("Error: {}", e);
                            break;
                            // ::std::process::exit(1)
                        }
                        _ => {}
                    }

                    match port.read(serial_buf.as_mut_slice()) {
                        Ok(t) => {
                            if let Some(std) = stdout {
                                if std {
                                    io::stdout().write_all(&serial_buf[..t]).unwrap();
                                }
                            }
                            if let Some(ref mut log) = log_file {
                                log.write_all(&serial_buf[..t]).unwrap();
                            }

                            // let msg = std::str::from_utf8(&serial_buf[..t]).unwrap();
                            // println!("{:?}", msg);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => eprintln!("{:?}", e),
                    }

                    if let Some(ia) = interactive {
                        if ia {
                            print!("console> ");
                            std::io::stdout().flush().unwrap();

                            let stdin = std::io::stdin();
                            let mut line_buf = String::new();
                            match stdin.read_line(&mut line_buf) {
                                Ok(t) => {
                                    let line = line_buf[..t].trim_end().to_string() + "\n";
                                    line_buf.clear();
                                    match port.write(line.as_bytes()) {
                                        Ok(_) => {
                                            // print!("User input: {}", &line);
                                            std::io::stdout().flush().unwrap();
                                        }
                                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                                        Err(e) => eprintln!("{:?}", e),
                                    }
                                }
                                Err(e) => {
                                    std::mem::drop(&port);
                                    eprintln!("{:?}", e)
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open \"{}\". Error: {}", self.port_name, e);
                ::std::process::exit(1);
            }
        }

        Ok(())
    }
}

/// UWB serial connector
#[pymodule]
fn uwb_serial(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<UWB>()?;
    Ok(())
}
