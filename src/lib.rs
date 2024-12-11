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
/// # Examples
///
/// ```
/// let input = rusty_coffeemachine::get_input();
/// ```
pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Print a line
///
/// This function prints a line to the terminal
///
/// # Examples
///
/// ```
/// rusty_coffeemachine::print_line();
/// ```
pub fn print_line() {
    println!("{}", "~".repeat(LINE_AMOUNT))
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
    /// # Examples
    ///
    /// ```
    /// progress_bar.set_progress(50.0);
    /// ```
    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress;
    }

    /// Draw the progress bar
    ///
    /// This function draws the progress bar to the console
    ///
    /// # Examples
    ///
    /// ```
    /// progress_bar.draw();
    /// ```
    pub fn draw(&self) {
        let progress = (self.progress / self.max * 100.0) as usize;
        let bar = format!(
            "\r[{}{}] {}%",
            "=".repeat(progress / 2),
            " ".repeat(50 - progress / 2),
            progress
        );
        print!("{}", bar);
        std::io::stdout().flush().unwrap();
    }
}
