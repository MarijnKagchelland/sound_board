#[allow(deprecated)]
pub use std::{fs, fs::{File, OpenOptions}};
pub use std::io::{BufWriter, Write};

pub use colored::*;
pub use chrono::{prelude::*, format};

pub mod prelude {
    pub use crate::{debug, verbose, info, warning, error};
    pub use crate::log_system::log::print_and_save_log;
    pub use chrono::Local;
}

pub fn print_and_save_log(log_level: u8, log_msg: &str) {
    match log_level {
        0 => println!("{}", log_msg.red()),
        1 => println!("{}", log_msg.yellow()),
        2 => println!("{}", log_msg.green()),
        3 => println!("{}", log_msg.purple()),
        _ => println!("{}", log_msg.white()),
    }
    
    let date = format!("{}", Local::now().date_naive());
    let file_name = format!("log_data/{}.log", date);
    fs::create_dir_all("log_data").unwrap();
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_name).unwrap();
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", log_msg).expect("Failed to write to log file");
}


#[macro_export] macro_rules! error {
    ($($log:tt)*) => {{
        let run_time_ms = Local::now().time();
        let log_string = format!($($log)*);
        let formatted_log = format!(
            "{}: ({}) [{}, {}]: {}",
            "ERROR",
            run_time_ms,
            file!(),
            line!(),
            log_string
        );
        print_and_save_log(0, &formatted_log);
    }};
}


#[macro_export] macro_rules! warning {
    ($($log:tt)*) => {{
        let run_time_ms = Local::now().time();
        let log_string = format!($($log)*);
        let formatted_log = format!(
            "{}: ({}) [{}, {}]: {}",
            "WARNING",
            run_time_ms,
            file!(),
            line!(),
            log_string
        );
        print_and_save_log(1, &formatted_log);
    }};
}

#[macro_export] macro_rules! info {
    ($($log:tt)*) => {{
        let run_time_ms = Local::now().time();
        let log_string = format!($($log)*);
        let formatted_log = format!(
            "{}: ({}) [{}, {}]: {}",
            "INFO",
            run_time_ms,
            file!(),
            line!(),
            log_string
        );
        print_and_save_log(2, &formatted_log);
    }};
}

#[macro_export] macro_rules! debug {
    ($($log:tt)*) => {{
        let run_time_ms = Local::now().time();
        let log_string = format!($($log)*);
        let formatted_log = format!(
            "{}: ({}) [{}, {}]: {}",
            "DEBUG",
            run_time_ms,
            file!(),
            line!(),
            log_string
        );
        print_and_save_log(3, &formatted_log);
    }};
}

#[macro_export] macro_rules! verbose {
    ($($log:tt)*) => {{
        let run_time_ms = Local::now().time();
        let log_string = format!($($log)*);
        let formatted_log = format!(
            "{:?}: ({}) [{}, {}]: {}",
            "VERBOSE",
            run_time_ms,
            file!(),
            line!(),
            log_string
        );
        print_and_save_log(4, &formatted_log);
    }};
}
