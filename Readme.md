
# System Monitoring Tool

This is a simple system monitoring tool built with Rust that provides real-time insights into CPU, memory, and disk usage. The tool also periodically logs these metrics to a file for historical analysis.

## Features

- **Real-Time Monitoring**: Displays CPU usage, memory usage, and disk information in a terminal-based UI.
- **Periodic Logging**: Logs system metrics (CPU and memory usage) to a file (`metrics_log.txt`) every 5 seconds for historical tracking.
- **User-Friendly Interface**: Uses color-coded sections and a clean layout for easy readability.
- **Cross-Platform**: Compatible with Windows, Linux, and macOS systems.

## Technologies Used

- **Rust**: Main programming language for high performance and reliability.
- **Sysinfo**: Library for accessing system information (CPU, memory, and disk metrics).
- **Tokio**: Asynchronous runtime for handling periodic logging without blocking the UI.
- **TUI and Crossterm**: Libraries for creating a terminal-based user interface.
- **Chrono**: Used for formatting timestamps in logs.

## File Structure

```
system_monitor/
├── Cargo.toml         # Cargo manifest file to manage dependencies
├── src/
│   ├── main.rs        # Entry point of the application
│   ├── ui.rs          # Terminal UI setup and rendering
│   ├── metrics.rs     # System metrics fetching functions
│   ├── input.rs       # User input handling (e.g., for quitting)
└── └── logger.rs      # Logging metrics to file periodically
```

## Getting Started

### Prerequisites

- **Rust**: Make sure Rust and Cargo are installed. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **Dependencies**: This tool uses several libraries. The required dependencies are listed in `Cargo.toml`.

### Installation

1. Clone this repository:

   ```bash
   git clone <repository-url>
   cd system_monitor
   ```

2. Build the application:

   ```bash
   cargo build --release
   ```

3. Run the application:

   ```bash
   cargo run --release
   ```

## Usage

- **Real-Time Monitoring**:
  - The terminal UI displays CPU usage, memory usage, and disk information.
  - These metrics are updated every second.

- **Logging to File**:
  - Every 5 seconds, system metrics are logged to `metrics_log.txt` in the root directory.
  - Each log entry includes a timestamp, CPU usage, and memory usage (used and total).
  - Example entry in `metrics_log.txt`:
    ```
    Timestamp: 2024-10-26 15:34:45, CPU Usage: 15.23%, Memory Usage: 2048 MB / 4096 MB
    ```

- **Quit the Tool**:
  - Press **`q`** to exit the application.

## Code Overview

### `main.rs`
The main entry point that initializes the terminal, refreshes metrics, and sets up an asynchronous logging interval every 5 seconds.

### `ui.rs`
Handles terminal UI rendering using `tui` and `crossterm`, displaying CPU, memory, and disk metrics in real time.

### `metrics.rs`
Fetches system metrics using the `sysinfo` library, providing functions to retrieve CPU usage, total memory, used memory, and available disk space.

### `input.rs`
Handles user input (e.g., pressing `q` to exit).

### `logger.rs`
Logs system metrics to `metrics_log.txt` every 5 seconds with a timestamp, CPU usage, and memory usage.

## Cross-Verification of Metrics

To cross-verify the metrics recorded by this tool:
1. **Task Manager (Windows)**, **Activity Monitor (macOS)**, or **top/htop (Linux)** for real-time CPU and memory usage.
2. **Performance Monitor (Windows)** or **sar (Linux)** for historical data if available.

## Possible Future Enhancements

- **Network Usage Monitoring**: Display bandwidth usage per interface.
- **Threshold Alerts**: Notify the user if CPU or memory usage exceeds a defined limit.
- **Configuration File**: Allow users to customize logging frequency, thresholds, and other settings.
- **Process Monitoring**: Show top resource-consuming processes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
