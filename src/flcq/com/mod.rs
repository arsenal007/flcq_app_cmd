extern crate clap;
extern crate serialport;

use crate::flcq::eeprom::{self, C, F, L, Q, R, S, W};

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
    pub fn quartz_crefs(&self) -> (f64, f64, f64, f64) {
        (
            self.eeprom.q_cref(),
            self.eeprom.q_cswitch(),
            self.eeprom.q_cref1(),
            self.eeprom.q_cref2(),
        )
    }
}

mod calculation {
    pub fn frequency(prescaler: u8, tmr0: u8, overflows_array: [u8; 4]) -> f64 {
        let overflows: u32;
        unsafe {
            overflows = std::mem::transmute::<[u8; 4], u32>(overflows_array);
        }
        let prescaler_values = [1.0f64, 2.0f64, 4.0f64, 8.0f64, 16.0f64];
        println!(
            "Overflows: {}, PSC: {},  Count: {}",
            overflows,
            prescaler_values[(prescaler + 1u8) as usize],
            tmr0 as f64
        );
        prescaler_values[(prescaler + 1u8) as usize] * (256.0f64 * overflows as f64 + tmr0 as f64)
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
                                                calculation::frequency(
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
    fn set_lc_crefs_execute(&mut self, a: std::string::String, c: f64) {
        if a == "1" {
            self.eeprom.set_cref1(c);
        } else if a == "2" {
            self.eeprom.set_cref2(c);
        }
    }

    pub fn set_lc_crefs(&mut self, cref: f64) {
        let vec = vec!["1", "2", "3", "4", "5", "6"];
        println!("LC:");
        println!("  1) set as Cref1");
        println!("  2) Set as Cref2");
        if let Some(Answer::RESPONSE(ans)) = Question::new("please press [1..2], then Enter:")
            .acceptable(vec)
            .ask()
        {
            self.set_lc_crefs_execute(ans, cref);
        }
    }

    fn i_set_quartz_crefs(&mut self, a: std::string::String, c: f64) {
        if a == "1" {
            self.eeprom.set_cq(c);
        } else if a == "2" {
            self.eeprom.set_cswitch(c);
        } else if a == "3" {
            self.eeprom.set_q_cref1(c);
        } else if a == "4" {
            self.eeprom.set_q_cref2(c);
        }
    }

    pub fn set_quartz_crefs(&mut self, cref: f64) {
        let vec = vec!["1", "2", "3", "4"];
        println!("Quartz Crystal:");
        println!("  1) Cq - quartz crystal capacity");
        println!("  2) Cs - switch capacitor");
        println!("  3) set as Cref1");
        println!("  4) set as Cref2");

        if let Some(Answer::RESPONSE(ans)) = Question::new("please press [1..4], then Enter:")
            .acceptable(vec)
            .ask()
        {
            self.i_set_quartz_crefs(ans, cref);
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

pub fn open<T: std::fmt::Display + AsRef<std::ffi::OsStr> + ?Sized>(
    v: &T,
    calibration: bool,
) -> Flcq {
    Flcq::new(v, calibration)
}
