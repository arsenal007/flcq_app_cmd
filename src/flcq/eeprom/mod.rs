pub trait F {
    fn frequency_periode(&self) -> f64;
    fn frequency_calibration_temperature(&self) -> f64;
    fn frequency_count(&self) -> u8;
    fn set_frequency(&mut self, periode: f64, temperature: f64, count: u8) -> ();
}

pub trait L {
    fn c0_l0(&self) -> (f64, f64);
    fn set_c0_l0(&mut self, c0: f64, l0: f64) -> ();
}

pub trait C {
    fn cref1(&self) -> f64;
    fn cref2(&self) -> f64;
    fn set_cref1(&mut self, cref1: f64) -> ();
    fn set_cref2(&mut self, cref2: f64) -> ();
}

pub trait Q {
    fn q_cref(&self) -> f64;
    fn set_cq(&mut self, v: f64);
    fn q_cswitch(&self) -> f64;
    fn set_cswitch(&mut self, v: f64);
    fn q_cref1(&self) -> f64;
    fn set_q_cref1(&mut self, v: f64);
    fn q_cref2(&self) -> f64;
    fn set_q_cref2(&mut self, v: f64);
}

pub trait R {
    fn read(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> ();
}

pub trait W {
    fn write(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> ();
}

pub trait S {
    fn show(&self) -> ();
}

mod crystal_quartz {
    use super::{TEeprom, Q, R, S, W};
    pub struct Quartz {
        q_cref: f64,
        q_cswitch: f64,
        q_cref1: f64,
        q_cref2: f64,
    }

    impl Default for Quartz {
        fn default() -> Self {
            Self {
                q_cref: super::f64_default(),
                q_cswitch: super::f64_default(),
                q_cref1: super::f64_default(),
                q_cref2: super::f64_default(),
            }
        }
    }

    impl Q for Quartz {
        fn q_cref(&self) -> f64 {
            self.q_cref
        }

        fn set_cq(&mut self, v: f64) {
            self.q_cref = v;
        }

        fn q_cswitch(&self) -> f64 {
            self.q_cswitch
        }

        fn set_cswitch(&mut self, v: f64) {
            self.q_cswitch = v;
        }

        fn q_cref1(&self) -> f64 {
            self.q_cref1
        }

        fn set_q_cref1(&mut self, v: f64) {
            self.q_cref1 = v;
        }

        fn q_cref2(&self) -> f64 {
            self.q_cref2
        }

        fn set_q_cref2(&mut self, v: f64) {
            self.q_cref2 = v;
        }
    }

    impl R for Quartz {
        fn read(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
            // crystal quartz parasitic own capicatance between pins
            self.q_cref = TEeprom::read_f64(port, &TEeprom::Q_CREF);

            // calibration capicator for F1/F2
            self.q_cswitch = TEeprom::read_f64(port, &TEeprom::Q_CSWITCH);

            // capicator C1 in emmiter
            self.q_cref1 = TEeprom::read_f64(port, &TEeprom::Q_CREF1);

            // capicator C1 in emmiter
            self.q_cref2 = TEeprom::read_f64(port, &TEeprom::Q_CREF2);
        }
    }

    impl W for Quartz {
        fn write(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
            // crystal quartz parasitic own capicatance between pins
            TEeprom::write_f64(port, &TEeprom::Q_CREF, &self.q_cref);

            // calibration capicator for F1/F2
            TEeprom::write_f64(port, &TEeprom::Q_CSWITCH, &self.q_cswitch);

            // capicator C1 in emmiter
            TEeprom::write_f64(port, &TEeprom::Q_CREF1, &self.q_cref1);

            // capicator C1 in emmiter
            TEeprom::write_f64(port, &TEeprom::Q_CREF2, &self.q_cref2);
        }
    }

    impl S for Quartz {
        fn show(&self) -> () {
            println!("Quartz crystal: ");
            println!("  quartz crystal own capacity = {:?} pF", self.q_cref);
            println!(
                "  switch capacity in G3UUR schema = {:?} pF",
                self.q_cswitch
            );

            println!("   C1 capacity G3UUR = {:?} pF", self.q_cref1);
            println!("   C2 capacity G3UUR = {:?} pF", self.q_cref2);
        }
    }
}

struct Freq {
    frequency_periode: f64,
    frequency_calibration_temperature: f64,
    frequency_calibration_count: u8,
}

struct capacity {
    cref1: f64,
    n_cref1: u8,
    cref2: f64,
    n_cref2: u8,
}

struct induction {
    c0: f64,
    l0: f64,
    n_c0_l0: u8,
}

pub struct TEeprom {
    sn: u8,
    freq: Freq,
    c: capacity,
    l: induction,
    q: crystal_quartz::Quartz,
}

impl TEeprom {
    // addresses in eeprom
    const SERIAL_NUMBER: u8 = 0u8; // u8
    const FREQUENCY_PERIODE: u8 = 1u8; // 9
    const CALIBRATION_TEMPERATURE: u8 = Self::FREQUENCY_PERIODE + 8u8; // 49
    const N_CALIBRATION_FREQUENCY_COUNT: u8 = Self::CALIBRATION_TEMPERATURE + 8u8; // 82
    const C0_SUM: u8 = Self::N_CALIBRATION_FREQUENCY_COUNT + 1u8; // f64
    const L0_SUM: u8 = Self::C0_SUM + 8u8; // f64
    const C0_L0_N: u8 = Self::L0_SUM + 8u8; // f64
    const CREF1_SUM: u8 = Self::C0_L0_N + 1u8; // f64
    const CREF1_N: u8 = Self::CREF1_SUM + 8u8; // u8
    const CREF2_SUM: u8 = Self::CREF1_N + 1u8; // f64
    const CREF2_N: u8 = Self::CREF2_SUM + 8u8; // u8
    const Q_CREF: u8 = Self::CREF2_N + 1u8; // f64
    const Q_CSWITCH: u8 = Self::Q_CREF + 8u8; // f64
    const Q_CREF1: u8 = Self::Q_CSWITCH + 8u8; // f64
    const Q_CREF2: u8 = Self::Q_CREF1 + 8u8; // f64
    const SIZE: u8 = Self::Q_CREF2 + 8u8;
}

fn f64_default() -> f64 {
    f64::from_le_bytes([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF])
}

impl Default for Freq {
    fn default() -> Self {
        Self {
            frequency_periode: f64_default(),
            frequency_calibration_temperature: f64_default(),
            frequency_calibration_count: 0xFF,
        }
    }
}

impl Default for capacity {
    fn default() -> Self {
        Self {
            cref1: f64_default(),
            n_cref1: 0xFF,
            cref2: f64_default(),
            n_cref2: 0xFF,
        }
    }
}

impl Default for induction {
    fn default() -> Self {
        Self {
            c0: f64_default(),
            l0: f64_default(),
            n_c0_l0: 0xFF,
        }
    }
}

impl Default for TEeprom {
    fn default() -> Self {
        Self {
            sn: u8::default(),
            freq: Freq::default(),
            c: capacity::default(),
            l: induction::default(),
            q: crystal_quartz::Quartz::default(),
        }
    }
}

impl F for Freq {
    fn frequency_periode(&self) -> f64 {
        self.frequency_periode
    }

    fn frequency_calibration_temperature(&self) -> f64 {
        self.frequency_calibration_temperature
    }

    fn frequency_count(&self) -> u8 {
        self.frequency_calibration_count
    }

    fn set_frequency(&mut self, periode: f64, temperature: f64, count: u8) -> () {
        self.frequency_periode = periode;
        self.frequency_calibration_temperature = temperature;
        self.frequency_calibration_count = count;
    }
}

impl R for Freq {
    fn read(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
        // periode
        self.frequency_periode = TEeprom::read_f64(port, &TEeprom::FREQUENCY_PERIODE);

        // calibration temperature
        self.frequency_calibration_temperature =
            TEeprom::read_f64(port, &TEeprom::CALIBRATION_TEMPERATURE);

        // calibration count
        self.frequency_calibration_count =
            TEeprom::read_byte(port, &TEeprom::N_CALIBRATION_FREQUENCY_COUNT);
    }
}

impl W for Freq {
    fn write(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
        // periode
        TEeprom::write_f64(port, &TEeprom::FREQUENCY_PERIODE, &self.frequency_periode);

        // calibration temperature
        TEeprom::write_f64(
            port,
            &TEeprom::CALIBRATION_TEMPERATURE,
            &self.frequency_calibration_temperature,
        );

        // calibration count
        TEeprom::write_byte(
            port,
            &TEeprom::N_CALIBRATION_FREQUENCY_COUNT,
            &self.frequency_calibration_count,
        );
    }
}

impl S for Freq {
    fn show(&self) -> () {
        println!(
            "       periode = {:?},  count = {:?}, temperature = {:?}",
            self.frequency_periode,
            self.frequency_calibration_count,
            self.frequency_calibration_temperature
        );
    }
}

impl C for capacity {
    fn cref1(&self) -> f64 {
        self.cref1 / self.n_cref1 as f64
    }

    fn cref2(&self) -> f64 {
        self.cref2 / self.n_cref2 as f64
    }

    fn set_cref1(&mut self, cref1: f64) -> () {
        if self.n_cref1 == 0 || self.n_cref1 == 0xFF {
            self.cref1 = cref1;
            self.n_cref1 = 1u8;
        } else {
            self.cref1 += cref1;
            self.n_cref1 += 1u8;
        }
    }

    fn set_cref2(&mut self, cref2: f64) -> () {
        if self.n_cref2 == 0 || self.n_cref2 == 0xFF {
            self.cref2 = cref2;
            self.n_cref2 = 1u8;
        } else {
            self.cref2 += cref2;
            self.n_cref2 += 1u8;
        }
    }
}

impl R for capacity {
    fn read(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
        // CREF1
        {
            self.cref1 = TEeprom::read_f64(port, &TEeprom::CREF1_SUM);
            self.n_cref1 = TEeprom::read_byte(port, &TEeprom::CREF1_N);
        }
        // CREF2
        {
            self.cref2 = TEeprom::read_f64(port, &TEeprom::CREF2_SUM);
            self.n_cref2 = TEeprom::read_byte(port, &TEeprom::CREF2_N);
        }
    }
}

impl W for capacity {
    fn write(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
        // CREF1
        {
            TEeprom::write_f64(port, &TEeprom::CREF1_SUM, &self.cref1);
            TEeprom::write_byte(port, &TEeprom::CREF1_N, &self.n_cref1);
        }

        // CREF2
        {
            TEeprom::write_f64(port, &TEeprom::CREF2_SUM, &self.cref2);
            TEeprom::write_byte(port, &TEeprom::CREF2_N, &self.n_cref2);
        }
    }
}

impl S for capacity {
    fn show(&self) {
        println!("capacity Refs:");
        println!(
            "       Cref1 = {:?} pF,  N_cref1 = {:?}, Cref2 = {:?} pF, N_cref2 = {:?}",
            self.cref1, self.n_cref1, self.cref2, self.n_cref2
        );
    }
}

impl L for induction {
    fn c0_l0(&self) -> (f64, f64) {
        (self.c0 / self.n_c0_l0 as f64, self.l0 / self.n_c0_l0 as f64)
    }

    fn set_c0_l0(&mut self, c0: f64, l0: f64) -> () {
        if self.n_c0_l0 == 0 || self.n_c0_l0 == 0xFF {
            self.c0 = c0;
            self.l0 = l0;
            self.n_c0_l0 = 1u8;
        } else {
            self.c0 += c0;
            self.l0 += l0;
            self.n_c0_l0 += 1u8;
        }
    }
}

impl R for induction {
    fn read(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
        // C0, L0
        self.c0 = TEeprom::read_f64(port, &TEeprom::C0_SUM);

        self.l0 = TEeprom::read_f64(port, &TEeprom::L0_SUM);

        self.n_c0_l0 = TEeprom::read_byte(port, &TEeprom::C0_L0_N);
    }
}

impl W for induction {
    fn write(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
        // C0, L0
        TEeprom::write_f64(port, &TEeprom::C0_SUM, &self.c0);
        TEeprom::write_f64(port, &TEeprom::L0_SUM, &self.l0);

        // N
        TEeprom::write_byte(port, &TEeprom::C0_L0_N, &self.n_c0_l0);
    }
}

impl S for induction {
    fn show(&self) {
        println!("induction:");
        println!(
            "       C0 = {:?} pF,  L0 = {:?} µH, N = {:?}",
            self.c0, self.l0, self.n_c0_l0
        );
    }
}

impl F for TEeprom {
    fn frequency_count(&self) -> u8 {
        self.freq.frequency_count()
    }
    fn frequency_periode(&self) -> f64 {
        self.freq.frequency_periode()
    }
    fn frequency_calibration_temperature(&self) -> f64 {
        self.freq.frequency_calibration_temperature()
    }
    fn set_frequency(&mut self, periode: f64, temperature: f64, count: u8) -> () {
        self.freq.set_frequency(periode, temperature, count)
    }
}

impl C for TEeprom {
    fn cref1(&self) -> f64 {
        self.c.cref1()
    }

    fn cref2(&self) -> f64 {
        self.c.cref2()
    }

    fn set_cref1(&mut self, cref1: f64) -> () {
        self.c.set_cref1(cref1)
    }

    fn set_cref2(&mut self, cref2: f64) -> () {
        self.c.set_cref2(cref2)
    }
}

impl L for TEeprom {
    fn c0_l0(&self) -> (f64, f64) {
        self.l.c0_l0()
    }

    fn set_c0_l0(&mut self, c0: f64, l0: f64) -> () {
        self.l.set_c0_l0(c0, l0)
    }
}

impl S for TEeprom {
    fn show(&self) -> () {
        println!("USED EEPROM SIZE {:?} bytes", TEeprom::SIZE);
        self.freq.show();
        self.c.show();
        self.l.show();
        self.q.show();
    }
}

impl R for TEeprom {
    fn read(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
        // Frequency
        self.freq.read(port);
        // capicatance
        self.c.read(port);
        // induction
        self.l.read(port);
    }
}

impl W for TEeprom {
    fn write(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
        // Frequency
        self.freq.write(port);
        // capicatance
        self.c.write(port);
        // induction
        self.l.write(port);

        println!("writed");
    }
}

impl Q for TEeprom {
    fn q_cref(&self) -> f64 {
        self.q.q_cref()
    }

    fn set_cq(&mut self, v: f64) {
        self.q.set_cq(v);
    }

    fn q_cswitch(&self) -> f64 {
        self.q.q_cswitch()
    }

    fn set_cswitch(&mut self, v: f64) {
        self.q.set_cswitch(v);
    }

    fn q_cref1(&self) -> f64 {
        self.q.q_cref1()
    }

    fn set_q_cref1(&mut self, v: f64) {
        self.q.set_q_cref1(v);
    }

    fn q_cref2(&self) -> f64 {
        self.q.q_cref2()
    }

    fn set_q_cref2(&mut self, v: f64) {
        self.q.set_q_cref2(v);
    }
}

impl TEeprom {
    pub fn clear(&mut self, port: &mut Box<dyn serialport::SerialPort>) -> () {
        for i in 1..128 {
            let d: u8 = 0xFF;
            Self::write_byte(port, &i, &d);
        }
    }
}

use crate::flcq::timeout;

impl TEeprom {
    fn read_byte(port: &mut Box<dyn serialport::SerialPort>, address: &u8) -> u8 {
        let write_data = vec![0x05u8, *address, 0xFFu8, 0xFFu8];

        match port.write(&write_data) {
            Ok(_) => (),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => timeout::timeout(
                &port,
                &std::string::String::from(" [ eeprom read byte request ] "),
            ),
            Err(e) => eprintln!("{:?}", e),
        }
        let mut read_data = vec![0; 5];
        match port.read(&mut read_data) {
            Ok(_n) => {
                if read_data[0] == 0x04 && read_data[2] == *address && _n == 5 {
                    // println!("{:02X?} {:02X?}", read_data[2], read_data[1]);
                    read_data[1]
                } else {
                    eprintln!(
                        "return address is different as in read command {:}, {:}",
                        read_data[2], *address
                    );
                    0xFFu8
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                timeout::timeout(&port, &std::string::String::from(" [ eeprom read byte ] "));
                0xFFu8
            }
            Err(e) => {
                eprintln!("{:?}", e);
                0xFFu8
            }
        }
    }
}

impl TEeprom {
    pub fn write_byte(port: &mut Box<dyn serialport::SerialPort>, address: &u8, data: &u8) -> () {
        let write_data = vec![0x03u8, *data, *address, 0xFFu8, 0xFFu8];
        match port.write(&write_data) {
            Ok(_) => {
                let mut read_data = vec![0; 5];
                match port.read(&mut read_data) {
                    Ok(_n) => {
                        if read_data[0] == 0x04
                            && read_data[1] == *data
                            && read_data[2] == *address
                            && _n == 5
                        {
                            ()
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => timeout::timeout(
                        &port,
                        &std::string::String::from(" [eeprom write byte respond ] "),
                    ),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => timeout::timeout(
                &port,
                &std::string::String::from(" [eeprom write byte request ] "),
            ),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

impl TEeprom {
    fn read_f64(port: &mut Box<dyn serialport::SerialPort>, _adrress: &u8) -> f64 {
        unsafe {
            let mut _byte_array = [0u8; 8];

            for i in 0..=7 {
                let adrress = *_adrress + i as u8;
                _byte_array[i] = Self::read_byte(port, &adrress);
            }
            std::mem::transmute::<[u8; 8], f64>(_byte_array)
        }
    }
}

impl TEeprom {
    fn write_f64(port: &mut Box<dyn serialport::SerialPort>, _adrress: &u8, _value: &f64) -> () {
        let b = _value.clone();
        let _byte_array: [u8; 8];
        unsafe {
            _byte_array = std::mem::transmute::<f64, [u8; 8]>(b);
        }
        for (i, item) in _byte_array.iter().enumerate() {
            let adrress = *_adrress + i as u8;
            Self::write_byte(port, &adrress, &item);
        }
    }
}
