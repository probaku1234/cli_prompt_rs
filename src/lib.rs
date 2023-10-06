//! `cli_prompts_rs` is a collection of prompt functions
//! to build CLI apps with nicely formatted output.
//!
//! # Example
//! ```no_run
//! use cli_prompts_rs::{CliPrompt, LogType, PromptSelectOption};
//! use std::process::exit;
//!
//! fn main() {
//!     let mut cli_prompt = CliPrompt::new();
//!     cli_prompt.intro("example app").unwrap();
//!
//!     cli_prompt.prompt_text("Enter your name").unwrap();
//!
//!     let answer = cli_prompt.prompt_confirm("Are you sure?").unwrap();
//!
//!     if !answer {
//!         cli_prompt.cancel("Operation cancelled").unwrap();
//!         exit(0);
//!     }
//!
//!     let options = vec![
//!         PromptSelectOption::new("option1", "Pikachu"),
//!         PromptSelectOption::new("option2", "Charmander"),
//!         PromptSelectOption::new("option3", "Squirtle"),
//!     ];
//!     let selected_option = cli_prompt
//!         .prompt_select("Which one do you prefer?", options)
//!         .unwrap();
//!
//!     cli_prompt
//!         .log(&format!("{}", selected_option), LogType::Info)
//!         .unwrap();
//!     cli_prompt.outro("Good Bye").unwrap();
//! }
//! ```
pub mod color;

use colored::*;
#[cfg(feature = "mock-term")]
use console::style;
#[cfg(not(feature = "mock-term"))]
use console::{style, Key, Term};
#[cfg(feature = "mock-term")]
use my_own_socket::{Key, Term};
use std::fmt;
use std::io::{Result, Write};
use supports_unicode::Stream;

fn get_symbol(c: &str, fallback: &str, unicode_support: bool) -> String {
    if unicode_support {
        return c.to_string();
    } else {
        return fallback.to_string();
    }
}

pub struct CliPrompt {
    term: Term,
    s_bar_start: String,
    s_bar: String,
    s_bar_end: String,
    s_radio_active: String,
    s_radio_inactive: String,
    s_step_submit: String,
    s_info: String,
    // s_success: String,
    s_warn: String,
    s_error: String,
}

impl CliPrompt {
    pub fn new() -> Self {
        let unicode_support = supports_unicode::on(Stream::Stdout);
        Self {
            term: Term::stdout(),
            s_bar_start: get_symbol("┌", "T", unicode_support),
            s_bar: get_symbol("│", "|", unicode_support),
            s_bar_end: get_symbol("└", "—", unicode_support),
            s_radio_active: get_symbol("●", ">", unicode_support),
            s_radio_inactive: get_symbol("○", " ", unicode_support),
            s_step_submit: get_symbol("◇", "o", unicode_support),
            s_info: get_symbol("●", "•", unicode_support),
            // s_success: get_symbol("◆", "*", unicode_support),
            s_warn: get_symbol("▲", "!", unicode_support),
            s_error: get_symbol("■", "x", unicode_support),
        }
    }

    /// Prints the intro message.
    ///
    /// Recommends to use at the beginning of your app.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    ///
    /// let mut cli_prompt = cli_prompts_rs::CliPrompt::new();
    /// cli_prompt.intro("example app").unwrap();
    /// ```
    pub fn intro(&mut self, message: &str) -> Result<()> {
        self.term
            .write_line(format!("{} {}", self.s_bar_start, message).as_str())?;

        Ok(())
    }

    /// Prints the outro message.
    ///
    /// Recommends to use at the end of your app.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    ///
    /// let mut cli_prompt = cli_prompts_rs::CliPrompt::new();
    /// cli_prompt.outro("example app").unwrap();
    /// ```
    pub fn outro(&mut self, message: &str) -> Result<()> {
        self.term
            .write_line(format!("{} {}", self.s_bar_end, message).as_str())?;

        Ok(())
    }

    /// Prints the cancel message with red color.
    ///
    /// Recommends to use when operation canceled and end your app.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    /// use std::process::exit;
    ///
    /// let mut cli_prompt = cli_prompts_rs::CliPrompt::new();
    /// let answer = cli_prompt.prompt_confirm("Are you sure?").unwrap();
    ///
    /// if !answer {
    ///     cli_prompt.cancel("Operation cancelled").unwrap();
    ///     exit(0);
    /// }
    /// ```
    pub fn cancel(&mut self, message: &str) -> Result<()> {
        self.term
            .write_line(format!("{} {}", self.s_bar_end, style(message).red()).as_str())?;
        Ok(())
    }

    /// Prints the log message with the corresponding symbols and color depends on the log type.
    ///
    /// Must provide the log type with [`LogType`] enum.
    ///
    /// # Format
    /// - Info: prefix symbol ● / color: blue
    /// - Warn: prefix symbol ▲ / color: yellow
    /// - Error: prefix symbol ■ / color: red
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::{CliPrompt, LogType};
    ///
    /// let mut cli_prompt = cli_prompts_rs::CliPrompt::new();
    /// cli_prompt.log("example log message", LogType::Info).unwrap();
    /// ```
    pub fn log(&mut self, message: &str, log_type: LogType) -> Result<()> {
        match log_type {
            LogType::Info => {
                self.term
                    .write_line(&format!("{} {}", self.s_info.blue(), message))?
            }
            LogType::Warn => {
                self.term
                    .write_line(&format!("{} {}", self.s_warn.yellow(), message.yellow()))?
            }
            LogType::Error => {
                self.term
                    .write_line(&format!("{} {}", self.s_error.red(), message.red()))?
            }
        }

        Ok(())
    }

    /// Prints the prompt message and read user's input
    ///
    /// Returns the input as `String` wrapped in `Result`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    ///
    /// let mut cli_prompt = cli_prompts_rs::CliPrompt::new();
    /// let answer = cli_prompt.prompt_text("example app").unwrap();
    /// println!("{}", answer);
    /// ```
    pub fn prompt_text(&mut self, message: &str) -> Result<String> {
        self.term
            .write_line(&self.format_prefix(message.to_string(), MessageType::Question))?;
        self.term.write(format!("{} ", self.s_bar).as_bytes())?;

        let line = self.term.read_line().unwrap();

        Ok(line.trim().to_string())
    }

    /// Prints the prompt message and let users to choose either yes or no.
    /// Users can change the selection by Arrow Left and Arrow Right key
    /// and choose the selection by Enter key.
    ///
    /// Returns true if user choose Yes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    ///
    /// let mut cli_prompt = cli_prompts_rs::CliPrompt::new();
    /// let answer = cli_prompt.prompt_confirm("Are you sure?").unwrap();
    /// println!("{}", answer);
    /// ```
    pub fn prompt_confirm(&mut self, message: &str) -> Result<bool> {
        let mut choice = 1;
        self.term.hide_cursor()?;
        self.term
            .write_line(&self.format_prefix(message.to_string(), MessageType::Question))?;
        self.print_confirm_message(true)?;

        loop {
            let key = self.term.read_key()?;

            match key {
                Key::ArrowLeft => {
                    self.print_confirm_message(true)?;
                    self.term.flush()?;
                    choice = 1;
                }
                Key::ArrowRight => {
                    self.print_confirm_message(false)?;
                    self.term.flush()?;
                    choice = 0;
                }
                Key::Enter => {
                    self.term.show_cursor()?;
                    self.term.write_line("")?;
                    break;
                }
                _ => {}
            }
        }

        return Ok(choice == 1);
    }

    /// Prints the prompt message and let users to choose one among the provided options.
    /// Users can change the selection by Arrow Up and Arrow down key
    /// and choose the selection by Enter key.
    ///
    /// Returns the selected option as instance of [`PromptSelectOption`] wrapped in `Result`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::{CliPrompt, PromptSelectOption};
    ///
    /// let mut cli_prompt = cli_prompts_rs::CliPrompt::new();
    /// let options = vec![
    ///     PromptSelectOption::new("option1", "Pikachu"),
    ///     PromptSelectOption::new("option2", "Charmander"),
    ///      PromptSelectOption::new("option3", "Squirtle"),
    /// ];
    /// let selected_option = cli_prompt.prompt_select("Which one do you prefer?", options).unwrap();
    /// println!("{}", selected_option);
    /// ```
    pub fn prompt_select(
        &mut self,
        message: &str,
        options: Vec<PromptSelectOption>,
    ) -> Result<PromptSelectOption> {
        let mut choice = 0;
        let options_num = options.len();
        self.term.hide_cursor()?;
        self.term
            .write_line(&self.format_prefix(message.to_string(), MessageType::Question))?;

        for i in 0..options_num {
            let current_option = &options.get(i).unwrap().label;
            if i == 0 {
                self.term.write_line(&self.format_prefix(
                    format!("{} {}", self.s_radio_active.green(), current_option),
                    MessageType::Option,
                ))?;
            } else {
                self.term.write_line(&self.format_prefix(
                    format!("{} {}", self.s_radio_inactive, current_option),
                    MessageType::Option,
                ))?;
            }
        }
        self.term.move_cursor_up(options_num)?;

        loop {
            let key = self.term.read_key()?;

            match key {
                Key::ArrowUp => {
                    choice = if choice == 0 {
                        options_num - 1
                    } else {
                        choice - 1
                    };

                    self.print_options(&options, choice)?;
                    self.term.flush()?;
                    self.term.move_cursor_up(options_num)?;
                }
                Key::ArrowDown => {
                    choice = (choice + 1) % options_num;

                    self.print_options(&options, choice)?;
                    self.term.flush()?;
                    self.term.move_cursor_up(options_num)?;
                }
                Key::Enter => {
                    self.term.move_cursor_down(options_num)?;
                    self.term.show_cursor()?;

                    break;
                }
                _ => {}
            }
        }
        Ok(options.get(choice).unwrap().clone())
    }

    fn format_prefix(&self, message: String, message_type: MessageType) -> String {
        return match message_type {
            MessageType::Question => {
                format!("{} {}", self.s_step_submit.magenta(), message)
            }
            MessageType::Option => format!("\r{} {}", self.s_bar, message),
        };
    }

    fn print_confirm_message(&mut self, is_yes: bool) -> Result<()> {
        if is_yes {
            self.term.write(
                self.format_prefix(
                    format!(
                        "{} Yes / {} No",
                        self.s_radio_active.green(),
                        self.s_radio_inactive
                    ),
                    MessageType::Option,
                )
                    .as_bytes(),
            )?;
        } else {
            self.term.write(
                self.format_prefix(
                    format!(
                        "{} Yes / {} No",
                        self.s_radio_inactive,
                        self.s_radio_active.green()
                    ),
                    MessageType::Option,
                )
                    .as_bytes(),
            )?;
        }

        Ok(())
    }

    fn print_options(
        &mut self,
        options: &Vec<PromptSelectOption>,
        current_choice: usize,
    ) -> Result<()> {
        for i in 0..options.len() {
            let current_option = &options.get(i).unwrap().label;

            if i == current_choice {
                self.term.write_line(&self.format_prefix(
                    format!("{} {}", self.s_radio_active.green(), current_option),
                    MessageType::Option,
                ))?;
            } else {
                self.term.write_line(&self.format_prefix(
                    format!("{} {}", self.s_radio_inactive, current_option),
                    MessageType::Option,
                ))?;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    #[cfg(feature = "mock-term")]
    fn get_term_input(&self) -> Vec<u8> {
        self.term.get_input()
    }

    #[allow(dead_code)]
    #[cfg(feature = "mock-term")]
    fn get_term_output(&self) -> Vec<u8> {
        self.term.get_output()
    }

    #[allow(dead_code)]
    #[cfg(feature = "mock-term")]
    fn set_term_input(&mut self, input: &str) {
        self.term.input = input.to_string().into_bytes();
    }

    #[allow(dead_code)]
    #[cfg(feature = "mock-term")]
    fn clear_term_output(&mut self) {
        self.term.output.clear();
    }

    #[allow(dead_code)]
    #[cfg(feature = "mock-term")]
    fn push_key_input(&mut self, key: &str) {
        self.term.key_input.push_back(key.to_string());
    }
}

enum MessageType {
    Question,
    Option,
}

/// Use to indicate the type of log for [`CliPrompt::log()`]
///
/// # Support Types
/// - Info
/// - Warn
/// - Error
pub enum LogType {
    Info,
    Warn,
    Error,
}

/// Use to define options for [`CliPrompt::prompt_select()`]
#[derive(Debug, Clone)]
pub struct PromptSelectOption {
    pub value: String,
    pub label: String,
}

impl PromptSelectOption {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
        }
    }
}

impl fmt::Display for PromptSelectOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.value, self.label)
    }
}

mod my_own_socket {
    use std::collections::VecDeque;
    use std::io::Write;

    pub struct Term {
        pub input: Vec<u8>,
        pub output: Vec<u8>,
        pub key_input: VecDeque<String>,
    }

    #[allow(dead_code)]
    impl Term {
        pub fn stdout() -> Self {
            Self {
                input: vec![],
                output: vec![],
                key_input: VecDeque::new(),
            }
        }

        pub fn write_line(&mut self, s: &str) -> Result<(), std::io::Error> {
            self.output.append(&mut s.to_string().into_bytes());
            self.output.push(b'\n');

            Ok(())
        }

        pub fn get_input(&self) -> Vec<u8> {
            self.input.clone()
        }

        pub fn get_output(&self) -> Vec<u8> {
            self.output.clone()
        }

        pub fn show_cursor(&self) -> Result<(), std::io::Error> {
            Ok(())
        }

        pub fn hide_cursor(&self) -> Result<(), std::io::Error> {
            Ok(())
        }

        pub fn read_key(&mut self) -> Result<Key, std::io::Error> {
            let input_key_option = self.key_input.pop_front();

            if input_key_option.is_none() {
                return Ok(Key::Enter);
            }

            match input_key_option.unwrap().as_str() {
                "arrow left" => Ok(Key::ArrowLeft),
                "arrow right" => Ok(Key::ArrowRight),
                "arrow down" => Ok(Key::ArrowDown),
                "arrow up" => Ok(Key::ArrowUp),
                "enter" => Ok(Key::Enter),
                _ => Ok(Key::Unknown),
            }
        }

        pub fn read_line(&self) -> Result<String, std::io::Error> {
            Ok(String::from_utf8(self.input.clone()).unwrap())
        }

        pub fn move_cursor_down(&self, _: usize) -> Result<(), std::io::Error> {
            Ok(())
        }

        pub fn move_cursor_up(&self, _: usize) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl Write for Term {
        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            self.output.append(&mut buf.to_vec());

            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    #[allow(dead_code)]
    #[derive(PartialEq, Eq, Hash)]
    pub enum Key {
        Unknown,
        /// Unrecognized sequence containing Esc and a list of chars
        UnknownEscSeq(Vec<char>),
        ArrowLeft,
        ArrowRight,
        ArrowUp,
        ArrowDown,
        Enter,
        Escape,
        Backspace,
        Home,
        End,
        Tab,
        BackTab,
        Alt,
        Del,
        Shift,
        Insert,
        PageUp,
        PageDown,
        Char(char),
    }
}

// #[cfg(feature = "mock-term")]
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn build_prefix_map() -> HashMap<String, String> {
        let unicode_support = supports_unicode::on(Stream::Stdout);
        let mut prefix_map = HashMap::new();

        prefix_map.insert(
            "s_bar_start".to_owned(),
            get_symbol("┌", "T", unicode_support),
        );
        prefix_map.insert("s_bar".to_owned(), get_symbol("│", "|", unicode_support));
        prefix_map.insert(
            "s_bar_end".to_owned(),
            get_symbol("└", "—", unicode_support),
        );
        prefix_map.insert(
            "s_radio_active".to_owned(),
            get_symbol("●", ">", unicode_support),
        );
        prefix_map.insert(
            "s_radio_inactive".to_owned(),
            get_symbol("○", " ", unicode_support),
        );
        prefix_map.insert(
            "s_step_submit".to_owned(),
            get_symbol("◇", "o", unicode_support),
        );
        prefix_map.insert("s_info".to_owned(), get_symbol("●", "•", unicode_support));
        prefix_map.insert("s_warn".to_owned(), get_symbol("▲", "!", unicode_support));
        prefix_map.insert("s_error".to_owned(), get_symbol("■", "x", unicode_support));

        prefix_map
    }

    #[test]
    fn test_get_symbol_works() {
        let unicode_symbol = get_symbol("c", "fallback", true);
        let fallback_symbol = get_symbol("c", "fallback", false);

        assert_eq!(unicode_symbol, "c");
        assert_eq!(fallback_symbol, "fallback");
    }

    #[test]
    fn test_format_prefix_question() {
        let cli_prompt = CliPrompt::new();
        let result = cli_prompt.format_prefix("test message".to_string(), MessageType::Question);

        let unicode_support = supports_unicode::on(Stream::Stdout);
        let prefix = get_symbol("◇", "o", unicode_support).magenta();
        assert_eq!(result, format!("{} {}", prefix, "test message"));
    }

    #[test]
    fn test_format_prefix_option() {
        let cli_prompt = CliPrompt::new();
        let result = cli_prompt.format_prefix("test message".to_string(), MessageType::Option);

        let unicode_support = supports_unicode::on(Stream::Stdout);
        let prefix = get_symbol("│", "|", unicode_support);
        assert_eq!(result, format!("\r{} {}", prefix, "test message"));
    }

    #[test]
    fn test_intro() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.intro("message").unwrap();

        let output = cli_prompt.get_term_output();
        let prefix_map = build_prefix_map();

        assert_eq!(
            format!("{} message\n", prefix_map.get("s_bar_start").unwrap()),
            String::from_utf8(output).unwrap()
        );
    }

    #[test]
    fn test_outro() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.outro("message").unwrap();

        let output = cli_prompt.get_term_output();
        let prefix_map = build_prefix_map();

        assert_eq!(
            format!("{} message\n", prefix_map.get("s_bar_end").unwrap()),
            String::from_utf8(output).unwrap()
        );
    }

    #[test]
    fn test_cancel() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.cancel("message").unwrap();

        let output = cli_prompt.get_term_output();
        let prefix_map = build_prefix_map();

        assert_eq!(
            format!(
                "{} {}\n",
                prefix_map.get("s_bar_end").unwrap(),
                style("message").red()
            ),
            String::from_utf8(output).unwrap()
        );
    }

    #[test]
    fn test_log() {
        let prefix_map = build_prefix_map();

        let mut cli_prompt = CliPrompt::new();
        cli_prompt.log("message", LogType::Info).unwrap();

        let mut output = cli_prompt.get_term_output();

        assert_eq!(
            format!(
                "{} {}\n",
                style(prefix_map.get("s_info").unwrap()).blue(),
                "message"
            ),
            String::from_utf8(output).unwrap()
        );

        cli_prompt.clear_term_output();
        cli_prompt.log("message", LogType::Warn).unwrap();
        output = cli_prompt.get_term_output();

        assert_eq!(
            format!(
                "{} {}\n",
                style(prefix_map.get("s_warn").unwrap()).yellow(),
                style("message").yellow()
            ),
            String::from_utf8(output).unwrap()
        );

        cli_prompt.clear_term_output();
        cli_prompt.log("message", LogType::Error).unwrap();
        output = cli_prompt.get_term_output();

        assert_eq!(
            format!(
                "{} {}\n",
                style(prefix_map.get("s_error").unwrap()).red(),
                style("message").red()
            ),
            String::from_utf8(output).unwrap()
        );
    }

    #[test]
    fn test_prompt_text() {
        let prefix_map = build_prefix_map();

        let mut cli_prompt = CliPrompt::new();

        cli_prompt.set_term_input("my name");

        let result = cli_prompt.prompt_text("name?").unwrap();

        let output = cli_prompt.get_term_output();

        assert_eq!(
            format!(
                "{} name?\n{} ",
                style(prefix_map.get("s_step_submit").unwrap()).magenta(),
                prefix_map.get("s_bar").unwrap()
            ),
            String::from_utf8(output).unwrap()
        );
        assert_eq!(result, "my name".to_string());
    }

    #[test]
    fn test_prompt_confirm_message() {
        let prefix_map = build_prefix_map();

        let mut cli_prompt = CliPrompt::new();

        cli_prompt.prompt_confirm("message").unwrap();

        let output = cli_prompt.get_term_output();

        assert_eq!(
            format!(
                "{} {}\n\r{} {} Yes / {} No\n",
                style(prefix_map.get("s_step_submit").unwrap()).magenta(),
                "message",
                prefix_map.get("s_bar").unwrap(),
                style(prefix_map.get("s_radio_active").unwrap()).green(),
                prefix_map.get("s_radio_inactive").unwrap()
            ),
            String::from_utf8(output).unwrap()
        );
    }

    #[test]
    fn test_prompt_confirm_yes() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.push_key_input("enter");

        let result = cli_prompt.prompt_confirm("message").unwrap();

        assert_eq!(result, true);
    }

    #[test]
    fn test_prompt_confirm_no() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.push_key_input("arrow right");
        cli_prompt.push_key_input("enter");

        let result = cli_prompt.prompt_confirm("message").unwrap();

        assert_eq!(result, false);
    }

    #[test]
    fn test_prompt_select_message() {
        let prefix_map = build_prefix_map();
        let mut cli_prompt = CliPrompt::new();
        let options = vec![
            PromptSelectOption::new("option1", "test option 1"),
            PromptSelectOption::new("option2", "test option 2"),
        ];
        cli_prompt.prompt_select("message", options).unwrap();

        let output = cli_prompt.get_term_output();

        assert_eq!(
            format!(
                "{} {}\n\
                \r{} {} {}\n\
                \r{} {} {}\n",
                style(prefix_map.get("s_step_submit").unwrap()).magenta(),
                "message",
                prefix_map.get("s_bar").unwrap(),
                style(prefix_map.get("s_radio_active").unwrap()).green(),
                "test option 1",
                prefix_map.get("s_bar").unwrap(),
                prefix_map.get("s_radio_inactive").unwrap(),
                "test option 2",
            ),
            String::from_utf8(output).unwrap()
        );
    }

    #[test]
    fn test_prompt_select_choose_option1() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.push_key_input("enter");
        let options = vec![
            PromptSelectOption::new("option1", "test option 1"),
            PromptSelectOption::new("option2", "test option 2"),
        ];

        let choice = cli_prompt.prompt_select("message", options).unwrap();

        assert_eq!(String::from("option1"), choice.value);
    }

    #[test]
    fn test_prompt_select_choose_option2() {
        let mut cli_prompt = CliPrompt::new();

        cli_prompt.push_key_input("arrow down");
        cli_prompt.push_key_input("enter");

        let options = vec![
            PromptSelectOption::new("option1", "test option 1"),
            PromptSelectOption::new("option2", "test option 2"),
        ];

        let choice = cli_prompt.prompt_select("message", options).unwrap();

        assert_eq!(String::from("option2"), choice.value);
    }
}
