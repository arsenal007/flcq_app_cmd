extern crate clap;
extern crate serialport;

//use std::io::{self, Write};
//use std::time::Duration;

//use serialport::prelude::*;
#[path = "calc.rs"]
mod calc;
#[path = "eeprom.rs"]
pub mod eeprom;
#[path = "serial_timeout.rs"]
mod timeout;

use self::eeprom::{C, F, L, R, S, W};

fn timeout_msg<T: std::fmt::Display>(
    port: &Box<dyn serialport::SerialPort>,
    s: &T,
) -> std::string::String {
    match port.name() {
        Some(name) => format!("{}: Timeout port \"{}\"", s, name),
        None => format!("\"{}\" port name is not avilable", s),
    }
}

impl S for Flcq {
    fn show(&self) {
        self.eeprom.show()
    }
}

impl Flcq {
    pub fn clear(&mut self) -> () {
        if let Some(p) = &mut self.port {
            self.eeprom.clear(p);
        }
    }
}

pub struct Flcq {
    port: Option<Box<dyn serialport::SerialPort>>,
    eeprom: Box<eeprom::TEeprom>,
    calibration: bool,
}

impl Flcq {
    fn new<T: std::fmt::Display + AsRef<std::ffi::OsStr> + ?Sized>(
        port_name: &T,
        calibration: bool,
    ) -> Self {
        match serialport::new(std::borrow::Cow::Owned(port_name.to_string()), 9600)
            .baud_rate(9600)
            .timeout(std::time::Duration::from_secs(60))
            .open()
        {
            Ok(mut result) => {
                let mut eeprom = Box::new(eeprom::TEeprom::default());
                eeprom.read(&mut result);

                let com = Flcq {
                    port: Some(result),
                    eeprom: eeprom,
                    calibration: calibration,
                };
                return com;
            }
            Err(e) => {
                eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
                ::std::process::exit(1);
            }
        }
    }
}

impl Flcq {
    pub fn disconnect(&mut self) {
        self.port = None;
    }
}

impl Flcq {
    pub fn is_init(&self) -> bool {
        match &self.port {
            Some(_) => true,
            None => false,
        }
    }
}

impl Flcq {
    fn temperature1(&self, _first: u8, _second: u8) -> f64 {
        let data = [_second, _first];
        unsafe {
            let raw = std::mem::transmute::<[u8; 2], u16>(data);
            let f = raw as f64;
            f * 0.0625
        }
    }
}

//temperature
impl Flcq {
    pub fn temperature(&mut self) -> (Option<f64>, std::string::String) {
        match &mut self.port {
            Some(port) => {
                let write_data = vec![0x09u8, 0x08u8, 0x00u8, 0xFFu8, 0xFFu8];

                match port.write(&write_data) {
                    Ok(_) => {
                        let mut read_data = vec![0; 5];
                        match port.read(&mut read_data) {
                            Ok(_n) => {
                                if read_data[0] == 0x0A && _n == 5 {
                                    (
                                        Some(self.temperature1(read_data[1], read_data[2])),
                                        "".to_string(),
                                    )
                                } else {
                                    (None, "Answer in Wrong format".to_string())
                                }
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (
                                None,
                                timeout_msg(
                                    &port,
                                    &std::string::String::from(
                                        " [ wait for temperature from FLCQ ] ",
                                    ),
                                ),
                            ),
                            Err(e) => (None, format!("{:?}", e)),
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (
                        None,
                        timeout_msg(
                            &port,
                            &std::string::String::from(" [ query for temperature from FLCQ ] "),
                        ),
                    ),
                    Err(e) => (None, format!("{:?}", e)),
                }
            }
            None => (None, "this should not be ever called".to_string()),
        }
    }
}

impl Flcq {
    fn frequency_count(&mut self, count: u8) -> Option<(f64, f64)> {
        let mut res = None;
        let freq = self.frequency_raw(&count);
        println!("self.frequency_raw(&count)");

        if let (Some(f), _) = freq {
            let (_prescaler, fl) = f;
            let true_count = self.eeprom.frequency_count();
            if count != true_count {
                return self.frequency_count(true_count);
            }
            let periode = self.eeprom.frequency_periode();
            let frequency = fl / periode;
            let calibration_temperature = self.eeprom.frequency_calibration_temperature();
            res = Some((calibration_temperature, frequency));
        }

        res
    }
}

impl Flcq {
    pub fn frequency(&mut self) -> Option<(f64, f64)> {
        let count = self.eeprom.frequency_count();
        self.frequency_count(count)
    }
}

impl Flcq {
    // continue frequency
    pub fn frequency_raw(&mut self, count: &u8) -> (Option<(u8, f64)>, std::string::String) {
        let n = count.clone();
        match &mut self.port {
            Some(port) => {
                if (0 < n) && (n < 255) {
                    let write_data = vec![0x0Bu8, 0x08u8, 0x10u8, n, 0xFFu8, 0xFFu8];
                    match port.write(&write_data) {
                        Ok(_) => {
                            let mut read_data = vec![0; 9];
                            match port.read(&mut read_data) {
                                Ok(_n) => {
                                    if read_data[0] == 0x06 && _n == 9 {
                                        (
                                            Some((
                                                read_data[1],
                                                calc::frequency(
                                                    read_data[1],
                                                    read_data[2],
                                                    [
                                                        read_data[3],
                                                        read_data[4],
                                                        read_data[5],
                                                        read_data[6],
                                                    ],
                                                ),
                                            )),
                                            "".to_string(),
                                        )
                                    } else {
                                        (None, "Answer in Wrong format".to_string())
                                    }
                                }
                                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (
                                    None,
                                    timeout_msg(
                                        &port,
                                        &std::string::String::from(
                                            " [ wait for temperature from FLCQ ] ",
                                        ),
                                    ),
                                ),
                                Err(e) => (None, format!("{:?}", e)),
                            }
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (
                            None,
                            timeout_msg(
                                &port,
                                &std::string::String::from(" [ query for frequency from FLCQ ] "),
                            ),
                        ),
                        Err(e) => (None, format!("{:?}", e)),
                    }
                } else {
                    (
                        None,
                        format!("wrong averging over {:?}, must be (0 < n < 255) ", n),
                    )
                }
            }
            None => (None, "this should not be ever called".to_string()),
        }
    }
}

impl Flcq {
    pub fn set_frequency(&mut self, period: f64, count: u8) {
        // println!("{:?}, {:?}, {:?}", period, prescaler, count);
        let temp = self.temperature();

        if let (Some(current_temperature), _) = temp {
            self.eeprom
                .set_frequency(period, current_temperature, count);
        }
    }
}

extern crate question;
use self::question::{Answer, Question};

impl Flcq {
    pub fn set_cref1_cref2(&mut self, cref: f64) {
        if let Some(Answer::YES) = Question::new("Set as CREF1")
            .yes_no()
            .until_acceptable()
            .default(Answer::YES)
            .show_defaults()
            .ask()
        {
            self.eeprom.set_cref1(cref);
        } else if let Some(Answer::YES) = Question::new("Set as CREF2")
            .yes_no()
            .until_acceptable()
            .default(Answer::YES)
            .show_defaults()
            .ask()
        {
            self.eeprom.set_cref2(cref);
        }
    }
}

impl Flcq {
    pub fn cref1(&mut self) -> f64 {
        self.eeprom.cref1()
    }
}

impl Flcq {
    pub fn c0_l0(&mut self) -> (f64, f64) {
        self.eeprom.c0_l0()
    }
}

impl Flcq {
    pub fn cref2(&mut self) -> f64 {
        self.eeprom.cref2()
    }
}

impl Flcq {
    pub fn set_c0_l0(&mut self, c0: f64, l0: f64) {
        self.eeprom.set_c0_l0(c0, l0);
    }
}

impl Drop for Flcq {
    fn drop(&mut self) {
        if self.calibration {
            if let Some(port) = &mut self.port {
                if let Some(Answer::YES) = Question::new("Save to EEPROM?")
                    .yes_no()
                    .until_acceptable()
                    .default(Answer::YES)
                    .show_defaults()
                    .ask()
                {
                    self.eeprom.write(port);
                }
            }
        }
    }
}

pub fn ports() -> std::result::Result<std::vec::Vec<serialport::SerialPortInfo>, serialport::Error>
{
    serialport::available_ports()
}

pub fn open<T: std::fmt::Display + AsRef<std::ffi::OsStr> + ?Sized>(
    v: &T,
    calibration: bool,
) -> Flcq {
    Flcq::new(v, calibration)
}

/*pub fn init() -> Flcq {
    Flcq {
        port: None,
        eeprom: Box::new(eeprom::TEeprom::default()),
    }
}*/
