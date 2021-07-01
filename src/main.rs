extern crate rustop;

use rustop::opts;

mod calibration;
mod measurements;

use self::measurements::lc::freq::com;
use self::measurements::lc::freq::com::eeprom::S;

fn usage() {
    println!("Usage examples:");
    println!("  calibration:");
    println!("    frequency: ./flcq.exe --calibration --frequency --frequency-periode-count 254 --com COM1 --ref-frequency 1000000");
    println!("    capacitance: ./flcq.exe --calibration --capacitance --com COM1 --cref1 1000 --cref2 47");
    println!(
        "    inductance: ./flcq.exe --calibration --inductance --com COM1 --cref1 1000 --cref2 47"
    );

    println!("  measurements:");
    println!("    frequency: ./flcq.exe --frequency --com COM1");
    println!("    capacitance: ./flcq.exe --capacitance --com COM1");
    println!("    inductance : ./flcq.exe  --inductance --com COM1 ");
}

fn main() {
    let parser = opts! {
        synopsis "FLCQ CMD program.";          // short info message for the help page
        auto_shorts false;
        version "version 0.1.1";
        // opt verbose:bool, desc:"Be verbose.";               // a flag -v or --verbose
        // opt luck:bool=true, desc:"We have no luck.";        // a flag -l or --no-luck
        // opt number_of_lines:usize=1, desc:"The number of lines.";                    // an option -n or --number-of-lines
        // param file:Option<String>, desc:"Input file name."; // an optional (positional) parameter
        opt com:Option<String>, desc:"COM Port string"; // an optional (positional) parameter
        opt calibration:bool, desc:"calibration";
        opt frequency:bool, desc:"frequency";
        opt capacitance:bool, desc:"capacitance";
        opt inductance:bool, desc:"inductance";
        opt show_saved:bool, desc:"show-saved";
        opt clear_saved:bool, desc:"clear-saved";
        opt quartz:bool, desc:"crystal quartz";
        opt cref1:Option<u16>, desc:"ref Capacitance 1";
        opt cref2:Option<u16>, desc:"ref Capacitance 2";
             opt frequency_periode_count:Option<u8>, desc:"frequency calibration periode count: max count, 1..254, one count approx 0.1048576 Sec";
        opt ref_frequency:Option<u32>, desc:"frequency calibration: ref frequency [kHz]";
    };

    usage();

    let (args, _rest) = parser.parse_or_exit();

    if let Some(com) = args.com {
        if args.calibration {
            calibration::calibration(
                com,
                args.frequency,
                args.capacitance,
                args.inductance,
                args.ref_frequency,
                args.frequency_periode_count,
                args.cref1,
                args.cref2,
            );
        } else if args.frequency {
            measurements::frequency(&com);
        } else if args.capacitance {
            measurements::capacitance(&com);
        } else if args.inductance {
            measurements::inductance(&com);
        } else if args.show_saved {
            let flcq: com::Flcq = com::open(&com, false);
            flcq.show();
        } else if args.clear_saved {
            let mut flcq: com::Flcq = com::open(&com, false);
            flcq.clear();
        }
    }
}
