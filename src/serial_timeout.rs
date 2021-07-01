
pub fn timeout<T: std::fmt::Display>(port: &Box<dyn serialport::SerialPort>, s: &T) -> () {
    match port.name() {
        Some(name) => println!("{}: Timeout port \"{}\"", s, name),
        None => println!("\"{}\" port name is not avilable", s),
    }
}
