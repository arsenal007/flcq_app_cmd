extern crate rustop;

use rustop::opts;
mod flcq {
    mod animation;
    pub mod calibration;
    pub mod capacity;
    pub mod com;
    mod converters;
    pub mod eeprom;
    mod f1f2;
    mod freq;
    mod lc;
    pub mod measurements;
    pub mod serial;
    mod timeout;
}

use flcq::eeprom::S;

fn usage() {
    println!("Usage examples:");
    println!("  calibration:");
    println!("    frequency: ./flcq.exe --calibration --frequency --frequency-periode-count 254 --ref-frequency 1000000");
    println!("    capacity: ./flcq.exe --calibration --capacity  --cref1 1000 --cref2 47");
    println!("    induction: ./flcq.exe --calibration --induction --cref1 1000 --cref2 47");
    println!("    induction: ./flcq.exe --calibration --quartz");

    println!("  measurements:");
    println!("    frequency: ./flcq.exe --frequency");
    println!("    capacity: ./flcq.exe --capacity");
    println!("    induction : ./flcq.exe  --induction");
    println!("    induction : ./flcq.exe  --quartz");
}

use crate::flcq::{calibration, com, measurements};

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
        opt capacity:bool, desc:"capacity";
        opt induction:bool, desc:"induction";
        opt show_saved:bool, desc:"show-saved";
        opt clear_saved:bool, desc:"clear-saved";
        opt quartz:bool, desc:"crystal quartz";
        opt quartz_capacity: bool, desc:"quartz capacity [Cq]";
        opt cref1:Option<u16>, desc:"ref capacity 1";
        opt cref2:Option<u16>, desc:"ref capacity 2";
             opt frequency_periode_count:Option<u8>, desc:"frequency calibration periode count: max count, 1..254, one count approx 0.1048576 Sec";
        opt ref_frequency:Option<u32>, desc:"frequency calibration: ref frequency [kHz]";
    };

    usage();

    let (args, _rest) = parser.parse_or_exit();

    if let Some((com, _hw, _fw)) = flcq::serial::Detect::run()
    //if let Some(com) = args.com
    {
        if args.calibration {
            calibration::calibration(
                com,
                args.frequency,
                args.capacity,
                args.induction,
                args.quartz,
                args.ref_frequency,
                args.frequency_periode_count,
                args.cref1,
                args.cref2,
            );
        } else if args.frequency {
            measurements::frequency(&com);
        } else if args.capacity {
            measurements::capacity(&com);
        } else if args.induction {
            measurements::induction(&com);
        } else if args.quartz {
            measurements::quartz(&com, args.quartz_capacity);
        } else if args.show_saved {
            let flcq = com::open(&com, false);
            flcq.show();
        } else if args.clear_saved {
            let mut flcq = com::open(&com, false);
            flcq.clear();
        }
    }
}
