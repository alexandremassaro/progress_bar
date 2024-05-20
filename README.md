# Progress Bar

`progress_bar` is a Rust library for displaying progress bars in the terminal. It uses `console_utils` for terminal interactions and can be used to track the progress of long-running operations.

## Features

- Display a progress bar in the terminal
- Update the progress bar dynamically
- Finish the progress bar when the operation is complete

## Dependencies

- `console_utils` for terminal interactions

## Usage

### Adding to Your Project

To use `progress_bar` in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
progress_bar = { path = "../progress_bar" }
```

### Example

```rust
use progress_bar::ProgressBar;
use std::result::Result;

fn main() -> Result<(), std::io::Error> {
    let total = 100;
    let mut progress_bar = ProgressBar::new(total)?;

    for i in 0..=total {
        progress_bar.update(i)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    progress_bar.finish()?;
    Ok(())
}
```

## License

This project is licensed under the MIT License.