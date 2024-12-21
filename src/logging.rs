use flexi_logger::{Age, Cleanup, Criterion, DeferredNow, Naming};
use log::Record;

use crate::{Result, CONFIG};

pub fn setup() -> Result<()> {
    use flexi_logger::{Duplicate, FileSpec, Logger};

    let log_dir = if let Some(ref dir) = CONFIG.log.dir {
        dir.to_string()
    } else {
        String::from("logs")
    };

    let mut log = Logger::try_with_str(&CONFIG.log.level)?
        .log_to_file(FileSpec::default().directory(log_dir))
        .format(my_format);

    if CONFIG.log.stdout {
        log = log.duplicate_to_stderr(Duplicate::All);
    }
    if cfg!(windows) {
        log = log.use_windows_line_ending();
    }

    if let Some(fileskept) = CONFIG.log.fileskept {
        log = log.rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(fileskept),
        )
    } else {
        log = log.rotate(Criterion::Age(Age::Day), Naming::Timestamps, Cleanup::Never);
    }

    let _log_handle = log.start()?;

    Ok(())
}

fn my_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    write!(
        w,
        "[{}] {} [{}:{}] {}",
        now.format(flexi_logger::TS_DASHES_BLANK_COLONS_DOT_BLANK),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        record.args(),
    )
}
