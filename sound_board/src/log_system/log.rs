#[allow(deprecated)]
pub use std::{fs, fs::{File, OpenOptions}};
pub use std::io::{BufWriter, Write};

pub use colored::*;
pub use chrono::{prelude::*, format};


#[derive(Debug)]
#[allow(dead_code)]
pub enum LogLevels {
    INFO,
    WARNING,
    ERROR,
    DEBUG,
    VERBOSE,
}

#[macro_export] macro_rules! info {
    ($($log:tt)*) => {{
        let log_level: LogLevels = LogLevels::INFO;
        let run_time_ms = Local::now().time();
        let date = format!("{}", Local::now().date_naive());
        let log_string = format!($($log)*);
        let formatted_log = format!(
            "{:?}: ({}) [{}, {}]: {}",
            log_level,
            run_time_ms,
            file!(),
            line!(),
            log_string
        );
        println!("{}", formatted_log.green());
        let file_name = format!("log_data/{}.log", date);
        let file =  OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_name).unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", formatted_log).expect("Failed to write to log file");
    }};
}

#[macro_export] macro_rules! warning {
    ($($log:tt)*) => {{
        let log_level: LogLevels = LogLevels::WARNING;
        let run_time_ms = Local::now();
        let date = format!("{}", Local::now().date_naive());
        let log_string = format!($($log)*);
        let formatted_log = format!(
            "{:?}: ({}) [{}, {}]: {}",
            log_level,
            run_time_ms,
            file!(),
            line!(),
            log_string
        );
        println!("{}", formatted_log.yellow());
        let file_name = format!("log_data/{}.log", date);
        let file =  OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_name).unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", formatted_log).expect("Failed to write to log file");
    }};
}

#[macro_export] macro_rules! error {
    ($($log:tt)*) => {{
        let log_level: LogLevels = LogLevels::ERROR;
        let run_time_ms = Local::now().time();
        let date = format!("{}", Local::now().date_naive());
        let log_string = format!($($log)*);
        let formatted_log = format!(
            "{:?}: ({}) [{}, {}]: {}",
            log_level,
            run_time_ms,
            file!(),
            line!(),
            log_string
        );
        println!("{}", formatted_log.red());
        let file_name = format!("log_data/{}.log", date);
        let file =  OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_name).unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", formatted_log).expect("Failed to write to log file");
    }};
}

#[macro_export] macro_rules! debug {
    ($($log:tt)*) => {{
        let log_level: LogLevels = LogLevels::DEBUG;
        let run_time_ms = Local::now().time();
        let date = format!("{}", Local::now().date_naive());
        let log_string = format!($($log)*);
        let formatted_log = format!(
            "{:?}: ({}) [{}, {}]: {}",
            log_level,
            run_time_ms,
            file!(),
            line!(),
            log_string
        );
        println!("{}", formatted_log.purple());
        let file_name = format!("log_data/{}.log", date);
        let file =  OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_name).unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", formatted_log).expect("Failed to write to log file");
    }};
}

#[macro_export] macro_rules! verbose {
    ($($log:tt)*) => {{
        let log_level: LogLevels = LogLevels::VERBOSE;
        let run_time_ms = Local::now().time();
        let date = format!("{}", Local::now().date_naive());
        let log_string = format!($($log)*);
        let formatted_log = format!(
            "{:?}: ({}) [{}, {}]: {}",
            log_level,
            run_time_ms,
            file!(),
            line!(),
            log_string
        );
        println!("{}", formatted_log.cyan());
        let file_name = format!("log_data/{}.log", date);
        let file =  OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_name).unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", formatted_log).expect("Failed to write to log file");
    }};
}
