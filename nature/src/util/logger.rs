use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

pub fn logger_init() {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "[{} {} {}:{}] {}",
                     Local::now().format("%Y-%m-%d %H:%M:%S"),
                     record.level(),
                     record.module_path().unwrap_or("=="),
                     record.line().unwrap_or(0),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();
}