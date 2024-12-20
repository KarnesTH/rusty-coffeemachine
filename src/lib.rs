pub mod coffeemachine;
pub mod containers;
pub mod reciepes;

use std::io::Write;

pub use coffeemachine::CoffeeMachine;
pub use containers::{GarbageContainer, IngredientsContainer};
pub use reciepes::Reciepes;

pub const LINE_AMOUNT: usize = 50;

/// Get input from the user
///
/// This function reads a line from the standard input and returns it as a string
///
/// # Returns
///
/// A string containing the input from the user
///
/// # Errors
///
/// This function will return an error if reading from the terminal fails
///
/// # Examples
///
/// ```
/// let input = rusty_coffeemachine::get_input();
/// ```
pub fn get_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Print a line
///
/// This function prints a line to the terminal
///
/// # Returns
///
/// An empty result
///
/// # Errors
///
/// This function will return an error if writing to the terminal fails
///
/// # Examples
///
/// ```
/// rusty_coffeemachine::print_line();
/// ```
pub fn print_line() -> Result<(), std::io::Error> {
    println!("{}", "~".repeat(LINE_AMOUNT));

    Ok(())
}

#[derive(Debug)]
pub struct ProgressBar {
    pub progress: f32,
    pub max: f32,
}

impl ProgressBar {
    /// Create a new progress bar
    ///
    /// # Arguments
    ///
    /// * `max` - The maximum value of the progress bar
    ///
    /// # Returns
    ///
    /// A new progress bar
    ///
    /// # Examples
    ///
    /// ```
    /// let progress_bar = rusty_coffeemachine::ProgressBar::new(100.0);
    /// ```
    pub fn new(max: f32) -> Self {
        ProgressBar { progress: 0.0, max }
    }

    /// Set the progress of the progress bar
    ///
    /// # Arguments
    ///
    /// * `progress` - The progress of the progress bar
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if the progress is greater than the max value
    ///
    /// # Examples
    ///
    /// ```
    /// let mut progress_bar = rusty_coffeemachine::ProgressBar::new(100.0);
    /// progress_bar.set_progress(50.0);
    /// ```
    pub fn set_progress(&mut self, progress: f32) -> Result<(), Box<dyn std::error::Error>> {
        if progress > self.max {
            return Err("Progress is greater than max value".into());
        }
        self.progress = progress;

        Ok(())
    }

    /// Draw the progress bar
    ///
    /// This function draws the progress bar to the console
    ///
    /// # Examples
    ///
    /// ```
    /// let progress_bar = rusty_coffeemachine::ProgressBar::new(100.0);
    /// progress_bar.draw();
    /// ```
    pub fn draw(&self) -> Result<(), std::io::Error> {
        let progress = (self.progress / self.max * 100.0) as usize;
        let bar = format!(
            "\r[{}{}] {}%",
            "=".repeat(progress / 2),
            " ".repeat(50 - progress / 2),
            progress
        );
        print!("{}", bar);
        std::io::stdout().flush()?;

        Ok(())
    }
}

/// Clean the terminal
///
/// This function clears the terminal
///
/// # Returns
///
/// An empty result
///
/// # Errors
///
/// This function will return an error if writing to the terminal fails
///
/// # Examples
///
/// ```
/// rusty_coffeemachine::clean_terminal();
/// ```
pub fn clean_terminal() -> Result<(), std::io::Error> {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_new() {
        let progress_bar = ProgressBar::new(100.0);
        assert_eq!(progress_bar.progress, 0.0);
        assert_eq!(progress_bar.max, 100.0);
    }

    #[test]
    fn test_progress_bar_set_progress() {
        let mut progress_bar = ProgressBar::new(100.0);
        progress_bar.set_progress(50.0).unwrap();
        assert_eq!(progress_bar.progress, 50.0);
    }

    #[test]
    fn test_progress_bar_set_progress_error() {
        let mut progress_bar = ProgressBar::new(100.0);
        let result = progress_bar.set_progress(150.0);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_progress_bar_edge_cases() {
        let mut progress_bar = ProgressBar::new(100.0);
        assert!(progress_bar.set_progress(0.0).is_ok());
        assert!(progress_bar.set_progress(100.0).is_ok());
    }
}
