use console_utils::{move_cursor_to, clear_current_line, get_cursor_position, print_formatted, print_newline};
use std::time::{Instant, Duration};
use std::io;
use std::result::Result;

pub const PROGRESS_BAR_WIDTH: usize = 50;

pub struct ProgressBar {
    total: usize,
    current: usize,
    line: u16,
    start_time: Instant,
    last_update: Instant,
    elapsed_per_unit: f64,
    last_message_len: usize,
}

impl ProgressBar {
    pub fn new(total: usize) -> Result<Self, io::Error> {
        let (_, line) = get_cursor_position()?;
        Ok(Self {
            total,
            current: 0,
            line,
            start_time: Instant::now(),
            last_update: Instant::now(),
            elapsed_per_unit: 0.0,
            last_message_len: 0,
        })
    }

    pub fn update(&mut self, value: usize) -> Result<(), io::Error> {
        let now = Instant::now();
        let elapsed_since_last = now.duration_since(self.last_update).as_secs_f64();
        let units_processed = (value - self.current) as f64;

        if units_processed > 0.0 {
            self.elapsed_per_unit = (self.elapsed_per_unit * self.current as f64 + elapsed_since_last) / value as f64;
        }

        self.current = value;
        self.last_update = now;
        self.print()
    }

    pub fn finish(&mut self) -> Result<(), io::Error> {
        self.current = self.total;
        self.print()?;
        print_newline()?;
        Ok(())
    }

    fn print(&mut self) -> Result<(), io::Error> {
        let percentage = (self.current as f64 / self.total as f64) * 100.0;
        let progress_bar = generate_progress_bar(percentage);
        let elapsed = self.start_time.elapsed();
        let estimated_total = self.elapsed_per_unit * self.total as f64;
        let remaining_secs = if estimated_total > elapsed.as_secs_f64() {
            estimated_total - elapsed.as_secs_f64()
        } else {
            0.0
        };
        let remaining = Duration::from_secs_f64(remaining_secs);
        let formatted_message = format!(
            "\rProgress: [{}] {:.2}% | Elapsed: {}s | Remaining: {}s",
            progress_bar,
            percentage,
            elapsed.as_secs(),
            remaining.as_secs()
        );

        // Move the cursor to the correct line
        move_cursor_to(self.line)?;

        // Only clear the line if the new message is shorter than the previous one
        if formatted_message.len() < self.last_message_len {
            clear_current_line()?;
        }

        print_formatted(&formatted_message)?;
        self.last_message_len = formatted_message.len();
        Ok(())
    }
}

fn generate_progress_bar(percentage: f64) -> String {
    let completed = (percentage / 100.0 * PROGRESS_BAR_WIDTH as f64) as usize;
    let remaining = PROGRESS_BAR_WIDTH - completed;
    format!("{}{}", "=".repeat(completed), " ".repeat(remaining))
}
