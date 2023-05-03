//rainfrog/src/rainfrog_logger.rs
use log::{Level, Metadata, Record, SetLoggerError};
use wasm_bindgen::prelude::*;


pub struct RainfrogLogger;

impl log::Log for RainfrogLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            web_sys::console::log_1(&format!("{:?}: {}", record.level(), record.args()).into());
        }
    }
    fn flush(&self) {}
}

impl RainfrogLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&RainfrogLogger)?;
        log::set_max_level(log::LevelFilter::Debug);
        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

pub fn console_log(s: String) {
    log(s);
}
