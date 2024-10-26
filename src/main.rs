mod ui;
mod metrics;
mod input;
mod logger;

use metrics::get_system_info;
use ui::draw_ui;
use input::handle_user_input;
use logger::log_metrics;
use tokio::time::{self, Duration};
use sysinfo:: SystemExt;  // Import SystemExt here

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ui::init_terminal()?;
    let mut system = get_system_info();

    // Set up an interval for periodic logging (e.g., every 5 seconds)
    let mut log_interval = time::interval(Duration::from_secs(5));

    loop {
        // Draw UI
        draw_ui(&mut terminal, &system);

        // Log metrics to file every 5 seconds
        log_interval.tick().await;
        if let Err(e) = log_metrics(&system) {
            eprintln!("Failed to log metrics: {}", e);
        }

        // Handle user input and exit on "q"
        if !handle_user_input() {
            break;
        }

        // Refresh system info for the next loop iteration
        tokio::time::sleep(Duration::from_secs(1)).await;
        system.refresh_all(); // This should work with SystemExt in scope
    }
    Ok(())
}
