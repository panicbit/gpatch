use std::fs::OpenOptions;
use std::sync::Mutex;
use std::env;
use std::io::{self, Write};

macro_rules! log {
    ($($args:expr),*) => ({
        writeln!($crate::log::LOG.lock().unwrap(), $($args),*).ok();
    })
}

lazy_static! {
    pub static ref LOG: Mutex<Box<Write + Send>> = {
        let log = env::var("GPATCH_LOG").ok().and_then(|path| {
            OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .ok()
            .map(|f| Box::new(f) as Box<Write + Send>)
        })
        .unwrap_or_else(|| Box::new(io::sink()));

        Mutex::new(log)
    };
}
