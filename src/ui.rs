use tui::widgets::{Block, Borders, Paragraph};
use tui::layout::{Layout, Constraint, Direction};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::text::{Span, Spans};
use tui::style::{Style, Color};
use sysinfo::{System, SystemExt, CpuExt, DiskExt};  // Import DiskExt for disk information
use std::io::{self, stdout};
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}};

/// Initializes the terminal with CrosstermBackend
pub fn init_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, io::Error> {
    let mut stdout = stdout();  // Make stdout mutable
    enable_raw_mode()?;  // Enables raw mode for capturing all key presses
    execute!(stdout, EnterAlternateScreen)?;  // Switch to alternate screen
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

/// Cleans up the terminal, restoring it to its normal state.
pub fn cleanup_terminal<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    disable_raw_mode()?;  // Disables raw mode
    let mut stdout = io::stdout();  // Make stdout mutable
    execute!(stdout, LeaveAlternateScreen)?;  // Exits alternate screen
    terminal.show_cursor()  // Shows the cursor again
}

/// Draws the system monitoring UI
pub fn draw_ui<B: tui::backend::Backend>(terminal: &mut Terminal<B>, system: &System) {
    terminal.draw(|frame| {
        // Divide the screen into three vertical sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)].as_ref())
            .split(frame.size());

        // Fetch and format CPU usage
        let cpu_usage: f32 = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / system.cpus().len() as f32;
        let cpu_info = Paragraph::new(Span::from(format!("CPU Usage: {:.2}%", cpu_usage)))
            .block(Block::default().title("CPU").borders(Borders::ALL))
            .style(Style::default().fg(Color::Green));

        // Fetch and format memory usage
        let used_memory = system.used_memory() / 1024;  // Convert to MB
        let total_memory = system.total_memory() / 1024;  // Convert to MB
        let mem_info = Paragraph::new(Span::from(format!("Memory Usage: {} MB / {} MB", used_memory, total_memory)))
            .block(Block::default().title("Memory").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));

        // Fetch and format disk usage for each disk
        let disk_info_text: Vec<Spans> = system.disks().iter().map(|disk| {
            let name = disk.name().to_string_lossy();
            let available = disk.available_space() / (1024 * 1024 * 1024);  // Convert to GB
            let total = disk.total_space() / (1024 * 1024 * 1024);  // Convert to GB
            Spans::from(format!("{}: {} GB / {} GB available", name, available, total))
        }).collect();

        let disk_info = Paragraph::new(disk_info_text)
            .block(Block::default().title("Disk").borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));

        // Render each section in the corresponding chunk
        frame.render_widget(cpu_info, chunks[0]);
        frame.render_widget(mem_info, chunks[1]);
        frame.render_widget(disk_info, chunks[2]);
    }).unwrap();
}
