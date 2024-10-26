use tui::widgets::{Block, Borders, Paragraph};
use tui::layout::{Layout, Constraint, Direction};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::text::Span;
use sysinfo::{System, SystemExt, CpuExt};  // Import CpuExt for CPU usage
use std::io::{self, stdout}; // Import `stdout` from `std::io`

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, io::Error> {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn draw_ui<B: tui::backend::Backend>(terminal: &mut Terminal<B>, system: &System) {
    terminal.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
            .split(frame.size());

        // Fetch CPU usage and display
        let cpu_usage: f32 = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / system.cpus().len() as f32;
        let cpu_info = Paragraph::new(Span::from(format!("CPU Usage: {:.2}%", cpu_usage)))
            .block(Block::default().title("CPU").borders(Borders::ALL));

        let mem_info = Paragraph::new(Span::from(format!("Memory Usage: {} / {}", system.used_memory() / 1024, system.total_memory() / 1024)))
            .block(Block::default().title("Memory").borders(Borders::ALL));

        let disk_info = Paragraph::new(Span::from("Disk Information"))
            .block(Block::default().title("Disk").borders(Borders::ALL));

        frame.render_widget(cpu_info, chunks[0]);
        frame.render_widget(mem_info, chunks[1]);
        frame.render_widget(disk_info, chunks[2]);
    }).unwrap();
}
