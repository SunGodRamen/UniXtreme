mod key_monitor;

use crate::key_monitor::KeyMonitor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let monitor = KeyMonitor::new()?;
    monitor.run()?;
    Ok(())
}

