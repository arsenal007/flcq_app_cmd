use std::io::Write;

pub struct Detect {
    i_ports: std::vec::Vec<(String, u8, u8)>,
    i_active_port_index: u8,
}

impl Default for Detect {
    fn default() -> Self {
        Self {
            i_ports: Vec::new(),
            i_active_port_index: 0x00,
        }
    }
}

impl Detect {
    pub fn run() -> std::option::Option<(std::string::String, u8, u8)> {
        let mut ports_container = Self::default();

        let mut animation = super::animation::Animation::new(std::string::String::from(
            "Search FLCQ device serial RS-232 [COM] port:",
        ));

        match serialport::available_ports() {
            Ok(ports) => {
                for port in ports {
                    let serial_port = &port.port_name;
                    match serialport::new(serial_port, 9600)
                        .baud_rate(9600)
                        .timeout(std::time::Duration::from_millis(50))
                        .open()
                    {
                        Ok(mut port) => match port.write(&vec![0x0Du8, 0x20, 0xFFu8, 0xFFu8]) {
                            Ok(_) => {
                                let mut data = vec![0; 4];
                                match port.read(&mut data) {
                                    Ok(_n) => {
                                        if data[0] == 0x0E && _n == 4 {
                                            let port = std::string::String::from(serial_port);
                                            let fw = data[1];
                                            let hw = data[2];

                                            println!(
                                                "found FLCQ on serial port: {}, fw: {}, hw: {}",
                                                serial_port, fw, hw
                                            );

                                            ports_container.i_ports.push((port, fw, hw));
                                        }
                                    }
                                    Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                                        let msg = std::string::String::from("serial port: ")
                                            + &serial_port
                                            + &std::string::String::from(" [timeout]");
                                        println!("{}", msg);
                                    }
                                    Err(e) => eprintln!("{:?}", e),
                                }
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                                let msg =
                                    std::string::String::from("ERROR: [0xFFFFFFFE] serial port: ")
                                        + &serial_port
                                        + &std::string::String::from("[write timeout]");
                                println!("{}", msg);
                            }
                            Err(e) => eprintln!("{:?}", e),
                        },
                        Err(e) => {
                            let serial_port = port.port_name.clone();
                            eprintln!(
                                "ERROR: [0xFFFFFFFD]. Failed to open \"{}\". Error: {}",
                                serial_port, e
                            );
                            //::std::process::exit(1);
                        }
                    }
                }
            }

            Err(_) => {
                println!("ERROR! 0xFFFFFFFF THERE IS NO AVIALBLE PORTS");
            }
        };

        if 1 == ports_container.i_ports.len() {
            ports_container.i_active_port_index = 0x00;
        } else if 1 < ports_container.i_ports.len() {
        }
        animation.end();
        Self::get_port(&ports_container)
    }

    // get port, HW, FW
    fn get_port(&self) -> std::option::Option<(std::string::String, u8, u8)> {
        if 0 < self.i_ports.len() {
            let (port, hw, fw) = &self.i_ports[self.i_active_port_index as usize];
            return Some((std::string::String::from(port), *hw, *fw));
        }
        None
    }
}
