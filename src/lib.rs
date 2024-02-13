// #![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
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
pub mod cli_prompt_error;
mod mock_term;
// pub mod color;
// pub mod spinner_error;

#[cfg(feature = "mock-term")]
use crate::mock_term::mock_term::{Key, Term};
use colored::*;
#[cfg(feature = "mock-term")]
use console::style;
#[cfg(not(feature = "mock-term"))]
use console::{style, Key, Term};
use std::fmt;
use std::io::{Result, Write};
use supports_unicode::Stream;

use crate::cli_prompt_error::CliPromptError::{
    self, InvalidMaxChoiceNumError, OptionsVecEmptyError,
};
#[cfg(feature = "unstable")]
use {
    crate::cli_prompt_error::SpinnerError,
    std::sync::mpsc::{self, TryRecvError},
    std::thread,
    std::time,
    std::time::Duration,
};

fn get_symbol(c: &str, fallback: &str, unicode_support: bool) -> String {
    return if unicode_support {
        c.to_string()
    } else {
        fallback.to_string()
    };
}

pub struct CliPrompt {
    term: Term,
    s_bar_start: String,
    s_bar: String,
    s_bar_h: String,
    s_bar_end: String,
    s_radio_active: String,
    s_radio_inactive: String,
    s_step_submit: String,
    s_info: String,
    #[cfg(feature = "unstable")]
    s_success: String,
    s_warn: String,
    s_error: String,
    s_corner_top_right: String,
    s_corner_bottom_right: String,
    s_connect_left: String,
    s_checkbox_active: String,
    s_checkbox_inactive: String,
    #[cfg(feature = "unstable")]
    s_spinner_frames: [String; 4],
}
// TODO: update doc parameter
// TODO: separate files
impl CliPrompt {
    pub fn new() -> Self {
        let unicode_support = supports_unicode::on(Stream::Stdout);
        Self {
            term: Term::stdout(),
            s_bar_start: get_symbol("┌", "T", unicode_support),
            s_bar: get_symbol("│", "|", unicode_support),
            s_bar_h: get_symbol("─", "-", unicode_support),
            s_bar_end: get_symbol("└", "—", unicode_support),
            s_radio_active: get_symbol("●", ">", unicode_support),
            s_radio_inactive: get_symbol("○", " ", unicode_support),
            s_step_submit: get_symbol("◇", "o", unicode_support),
            s_info: get_symbol("●", "•", unicode_support),
            #[cfg(feature = "unstable")]
            s_success: get_symbol("◆", "*", unicode_support),
            s_warn: get_symbol("▲", "!", unicode_support),
            s_error: get_symbol("■", "x", unicode_support),
            s_corner_top_right: get_symbol("╮", "+", unicode_support),
            s_corner_bottom_right: get_symbol("╯", "+", unicode_support),
            s_connect_left: get_symbol("├", "+", unicode_support),
            s_checkbox_active: get_symbol("◼", "[+]", unicode_support),
            s_checkbox_inactive: get_symbol("◻", "[ ]", unicode_support),
            #[cfg(feature = "unstable")]
            s_spinner_frames: [
                get_symbol("◒", "•", unicode_support),
                get_symbol("◐", "o", unicode_support),
                get_symbol("◓", "O", unicode_support),
                get_symbol("◑", "0", unicode_support),
            ],
        }
    }

    // pub fn new_with_config() -> Self {
    //
    // }

    /// Prints the intro message.
    ///
    /// Recommends to use at the beginning of your app.
    ///
    /// # Arguments
    ///
    /// * `message` - the intro message
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// cli_prompt.intro("example app").unwrap();
    /// ```
    pub fn intro(&mut self, message: &str) -> std::result::Result<(), CliPromptError> {
        self.term
            .write_line(format!("{} {}", self.s_bar_start, message).as_str())?;
        self.print_empty_line()?;

        Ok(())
    }

    /// Prints the outro message.
    ///
    /// Recommends to use at the end of your app.
    ///
    /// # Arguments
    ///
    /// * `message` - the outro message
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// cli_prompt.outro("example app").unwrap();
    /// ```
    pub fn outro(&mut self, message: &str) -> std::result::Result<(), CliPromptError> {
        self.term
            .write_line(format!("{} {}", self.s_bar_end, message).as_str())?;

        Ok(())
    }

    /// Prints the cancel message with red color.
    ///
    /// Recommends to use when operation canceled and end your app.
    ///
    /// # Arguments
    ///
    /// * `message` - the cancel message
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    /// use std::process::exit;
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// let answer = cli_prompt.prompt_confirm("Are you sure?").unwrap();
    ///
    /// if !answer {
    ///     cli_prompt.cancel("Operation cancelled").unwrap();
    ///     exit(0);
    /// }
    /// ```
    pub fn cancel(&mut self, message: &str) -> std::result::Result<(), CliPromptError> {
        self.term
            .write_line(format!("{} {}", self.s_bar_end, style(message).red()).as_str())?;
        Ok(())
    }

    /// Prints the log message with the corresponding symbols and color depends on the log type.
    ///
    /// Must provide the log type with [`LogType`] enum.
    ///
    /// # Arguments
    ///
    /// * `message` - the log message
    /// * `log_type` - the type of log
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
    /// let mut cli_prompt = CliPrompt::new();
    /// cli_prompt.log("example log message", LogType::Info).unwrap();
    /// ```
    pub fn log(
        &mut self,
        message: &str,
        log_type: LogType,
    ) -> std::result::Result<(), CliPromptError> {
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
        self.print_empty_line()?;

        Ok(())
    }

    /// Prints the prompt message and read user's input
    ///
    /// Returns the input as `String` wrapped in `Result`
    ///
    /// # Arguments
    ///
    /// * `message` - the prompt message
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// let answer = cli_prompt.prompt_text("example app").unwrap();
    /// println!("{}", answer);
    /// ```
    pub fn prompt_text(&mut self, message: &str) -> std::result::Result<String, CliPromptError> {
        self.term
            .write_line(&self.format_prefix(message.to_string(), MessageType::Question))?;
        self.term.write(format!("{} ", self.s_bar).as_bytes())?;

        let line = self.term.read_line()?;
        self.print_empty_line()?;

        Ok(line.trim().to_string())
    }

    /// Prints the prompt message and let users choose either yes or no.
    /// Users can change the selection by Arrow Left and Arrow Right key
    /// and choose the selection by Enter key.
    ///
    /// Returns true if user choose Yes.
    ///
    /// # Arguments
    ///
    /// * `message` - the prompt message
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// let answer = cli_prompt.prompt_confirm("Are you sure?").unwrap();
    /// println!("{}", answer);
    /// ```
    pub fn prompt_confirm(&mut self, message: &str) -> std::result::Result<bool, CliPromptError> {
        // TODO: when message is empty, get default message
        let prompt_message = if message.is_empty() {
            "Are you sure?"
        } else {
            message
        };
        let mut choice = 1;
        self.term.hide_cursor()?;
        self.term
            .write_line(&self.format_prefix(prompt_message.to_string(), MessageType::Question))?;
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
                    self.print_empty_line()?;
                    break;
                }
                _ => {}
            }
        }

        return Ok(choice == 1);
    }

    /// Prints the prompt message and let users choose one among the provided options.
    /// Users can change the selection by Arrow Up and Arrow down key
    /// and choose the selection by Enter key.
    ///
    /// Returns the selected option as instance of [`PromptSelectOption`] wrapped in `Result`.
    ///
    /// # Arguments
    ///
    /// * `message` - the prompt message
    /// * `options` - the list of options for selection
    ///
    /// # Errors
    ///
    /// If `options` is empty, [`OptionsVecEmptyError`](OptionsVecEmptyError) will be returned.
    ///
    /// ```
    /// use cli_prompts_rs::{CliPrompt, PromptSelectOption};
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// let options = vec![];
    ///
    /// let result = cli_prompt.prompt_select("Which one do you prefer?", options);
    /// assert!(result.is_err());
    /// ```
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::{CliPrompt, PromptSelectOption};
    ///
    /// let mut cli_prompt = CliPrompt::new();
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
    ) -> std::result::Result<PromptSelectOption, CliPromptError> {
        if options.is_empty() {
            return Err(OptionsVecEmptyError {
                message: "options is empty".to_string(),
            });
        }

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
                    self.print_empty_line()?;

                    break;
                }
                _ => {}
            }
        }
        Ok(options.get(choice).unwrap().clone())
    }

    /// Prints the prompt message and let users choose multiple options among the provided ones.
    /// Users can change the selection by Arrow Up and Arrow down key
    /// and choose the selection by Enter key.
    ///
    /// Returns the selected options as `Vector` of [`PromptSelectOption`] wrapped in `Result`.
    ///
    /// This function is same as calling [`CliPrompt::prompt_multi_select_with_max_choice_num`] with options length as `max_choice_num`.
    /// ```no_run
    /// use cli_prompts_rs::{CliPrompt, PromptSelectOption};
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// let options = vec![
    ///     PromptSelectOption::new("option1", "Pikachu"),
    ///     PromptSelectOption::new("option2", "Charmander"),
    ///      PromptSelectOption::new("option3", "Squirtle"),
    /// ];
    /// let options_len = options.len();
    /// cli_prompt.prompt_multi_select_with_max_choice_num("message", options, options_len).unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// * `message` - the prompt message
    /// * `options` - the list of options for selection
    ///
    /// # Errors
    ///
    /// If `options` is empty, [`OptionsVecEmptyError`](OptionsVecEmptyError) will be returned.
    ///
    /// ```
    /// use cli_prompts_rs::{CliPrompt, PromptSelectOption};
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// let options = vec![];
    ///
    /// let result = cli_prompt.prompt_multi_select("Which one do you prefer?", options);
    /// assert!(result.is_err());
    /// ```
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::{CliPrompt, PromptSelectOption};
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// let options = vec![
    ///     PromptSelectOption::new("option1", "Pikachu"),
    ///     PromptSelectOption::new("option2", "Charmander"),
    ///      PromptSelectOption::new("option3", "Squirtle"),
    /// ];
    /// let selected_options = cli_prompt.prompt_multi_select("Which one do you prefer?", options).unwrap();
    /// println!("{:?}", selected_options);
    pub fn prompt_multi_select(
        &mut self,
        message: &str,
        options: Vec<PromptSelectOption>,
    ) -> std::result::Result<Vec<PromptSelectOption>, CliPromptError> {
        let options_len = options.len();
        self.prompt_multi_select_with_max_choice_num(message, options, options_len)
    }

    /// Prints the prompt message and let users choose multiple options among the provided ones.
    /// Users can select up to `max_choice_num` from the options.
    /// Users can change the selection by Arrow Up and Arrow down key
    /// and choose the selection by Enter key.
    ///
    /// Returns the selected options as `Vector` of [`PromptSelectOption`] wrapped in `Result`.
    ///
    /// # Arguments
    ///
    /// * `message` - the prompt message
    /// * `options` - the list of options for selection
    /// * `max_choice_num` - the maximum number of choice. Must be greater than 0 and equal or less than length of `options`.
    ///
    /// # Errors
    ///
    /// If `options` is empty, [`OptionsVecEmptyError`](OptionsVecEmptyError) will be returned.
    ///
    /// ```
    /// use cli_prompts_rs::{CliPrompt, PromptSelectOption};
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// let options = vec![];
    ///
    /// let result = cli_prompt.prompt_multi_select("Which one do you prefer?", options);
    /// assert!(result.is_err());
    /// ```
    ///
    /// If `max_choice_num` is zero or greater than length of `options`, [`InvalidMaxChoiceNumber`](InvalidMaxChoiceNumError) will be returned.
    /// ```
    /// use cli_prompts_rs::{CliPrompt, PromptSelectOption};
    ///
    /// let mut cli_prompt = CliPrompt::new();
    ///
    /// let options = vec![
    ///     PromptSelectOption::new("option1", "test option 1"),
    ///     PromptSelectOption::new("option2", "test option 2"),
    /// ];
    ///
    /// let result = cli_prompt.prompt_multi_select_with_max_choice_num("message", options, 0);
    ///
    /// assert!(result.is_err());
    /// let error = result.unwrap_err();
    /// assert_eq!(error.to_string(), "max_choice_num must be greater than 0");
    /// ```
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::{CliPrompt, PromptSelectOption};
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// let options = vec![
    ///     PromptSelectOption::new("option1", "Pikachu"),
    ///     PromptSelectOption::new("option2", "Charmander"),
    ///      PromptSelectOption::new("option3", "Squirtle"),
    /// ];
    /// let selected_options = cli_prompt.prompt_multi_select("Which one do you prefer?", options).unwrap();
    /// println!("{:?}", selected_options);
    /// ```
    ///
    /// With empty `options`, it will return [`OptionsVecEmptyError`](OptionsVecEmptyError).
    pub fn prompt_multi_select_with_max_choice_num(
        &mut self,
        message: &str,
        options: Vec<PromptSelectOption>,
        max_choice_num: usize,
    ) -> std::result::Result<Vec<PromptSelectOption>, CliPromptError> {
        if options.is_empty() {
            return Err(OptionsVecEmptyError {
                message: "options is empty".to_string(),
            });
        }

        if max_choice_num > options.len() {
            return Err(InvalidMaxChoiceNumError {
                message: "max_choice_num must be less or equal than options length".to_string(),
            });
        }

        if max_choice_num == 0 {
            return Err(InvalidMaxChoiceNumError {
                message: "max_choice_num must be greater than 0".to_string(),
            });
        }

        let mut choice = 0;
        let mut current_selected_num = 0;
        let options_num = options.len();
        let mut is_selected = Vec::new();
        for _i in 0..options_num {
            is_selected.push(false);
        }
        self.term.hide_cursor()?;
        self.term
            .write_line(&self.format_prefix(message.to_string(), MessageType::Question))?;

        for i in 0..options_num {
            let current_option = &options.get(i).unwrap().label;
            if i == 0 {
                self.term.write_line(&self.format_prefix(
                    format!(
                        "{} {} {}",
                        self.s_radio_active.green(),
                        self.s_checkbox_inactive,
                        current_option
                    ),
                    MessageType::Option,
                ))?;
            } else {
                self.term.write_line(&self.format_prefix(
                    format!(
                        "{} {} {}",
                        self.s_radio_inactive, self.s_checkbox_inactive, current_option
                    ),
                    MessageType::Option,
                ))?;
            }
        }

        self.print_multi_select_confirm(options_num, choice)?;

        self.term.move_cursor_up(options_num + 1)?;

        loop {
            let key = self.term.read_key()?;

            match key {
                Key::ArrowUp => {
                    choice = if choice == 0 { options_num } else { choice - 1 };
                    // print options
                    self.print_multi_options(&options, &is_selected, choice)?;
                    self.term.flush()?;
                    self.term.move_cursor_up(options_num + 1)?;
                }
                Key::ArrowDown => {
                    choice = (choice + 1) % (options_num + 1);
                    // print options
                    self.print_multi_options(&options, &is_selected, choice)?;
                    self.term.flush()?;
                    self.term.move_cursor_up(options_num + 1)?;
                }
                Key::Enter => {
                    if choice == options_num {
                        self.term.move_cursor_down(options_num + 1)?;
                        self.term.show_cursor()?;
                        self.print_empty_line()?;

                        break;
                    } else {
                        match is_selected[choice] {
                            true => {
                                current_selected_num -= 1;
                                is_selected[choice] = !is_selected[choice];
                            }
                            false => {
                                if max_choice_num > current_selected_num {
                                    current_selected_num += 1;
                                    is_selected[choice] = !is_selected[choice];
                                }
                            }
                        }
                        self.print_multi_options(&options, &is_selected, choice)?;
                        self.term.flush()?;
                        self.term.move_cursor_up(options_num + 1)?;
                    }
                }
                _ => {}
            }
        }

        let mut selected_options = Vec::new();
        for i in 0..options_num {
            if is_selected[i] {
                selected_options.push(options[i].clone());
            }
        }

        Ok(selected_options)
    }

    /// Prints message wrapped by a box.
    ///
    /// # Arguments
    ///
    /// * `note_message` - the note message
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cli_prompts_rs::CliPrompt;
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// cli_prompt.print_note("example note").unwrap();
    ///
    /// let note_message = r#"
    /// Lorem ipsum dolor sit amet, consectetur adipiscing elit.
    /// Sed vel dui sit amet nibh accumsan imperdiet quis sed nibh.
    /// Cras commodo nisl in eros tincidunt sodales.
    /// "#;
    /// cli_prompt.print_note(note_message).unwrap();
    /// ```
    pub fn print_note(&mut self, note_message: &str) -> std::result::Result<(), CliPromptError> {
        // split message by \n
        let split_message = note_message.split("\n");
        // get max length of split messages
        let max_length_option = split_message.map(|m| m.len()).max();
        let max_length = max_length_option.unwrap_or_else(|| 0);

        // print header
        self.term.write_line(
            format!(
                "{}{}{}",
                self.s_connect_left,
                self.s_bar_h.repeat(max_length + 2),
                self.s_corner_top_right
            )
            .as_str(),
        )?;
        // print message
        for message in note_message.split("\n") {
            let message_length = message.len();
            self.term.write_line(
                format!(
                    "{} {}{}{}",
                    self.s_bar,
                    message,
                    " ".repeat(max_length - message_length + 1),
                    self.s_bar
                )
                .as_str(),
            )?;
        }

        // print footer
        self.term.write_line(
            format!(
                "{}{}{}",
                self.s_connect_left,
                self.s_bar_h.repeat(max_length + 2),
                self.s_corner_bottom_right
            )
            .as_str(),
        )?;

        Ok(self.print_empty_line()?)
    }

    /// Displays spinner while waiting for the `task` to finish.
    /// When this function is called, starts the spinner, spawns a new thread, and call `task`.
    /// Once the `task` is done, the spinner stopped, and prints the complete message.
    ///
    /// `timeout` is used to set maximum time for task to run.
    ///
    /// Since `task` is called inside a thread, It has two constraints, [`Send`] and `'static`, same as [`thread::spawn`]'s `f` closure.
    ///
    /// # Arguments
    ///
    /// * `loading_message` - the loading message
    /// * `finish_message` - the finish message that printed when `task` is done
    /// * `timeout` - the time limit in milliseconds
    /// * `task` - the closure that runs in thread, while spinner is running
    ///
    /// # Errors
    ///
    /// If `task` is not finished within given `timeout`, [`TimedOut`](SpinnerError::TimedOut) will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{thread, time};
    /// use cli_prompts_rs::CliPrompt;
    ///
    /// let mut cli_prompt = CliPrompt::new();
    /// let pika = || {
    ///     thread::sleep(time::Duration::from_millis(500));
    /// };
    /// cli_prompt.run_with_spinner("loading", "Done!", 5000, pika).unwrap();
    /// ```
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    pub fn run_with_spinner<F, T>(
        &mut self,
        loading_message: &str,
        finish_message: &str,
        timeout: u64,
        mut task: F,
    ) -> std::result::Result<(), CliPromptError>
    where
        F: FnMut() -> T,
        F: Send + 'static,
    {
        // TODO: thread error handle?
        // TODO: show result?
        // TODO: come up better function name
        let now = time::Instant::now();
        let mut spinner_symbol_index = 0;
        let (tx, rx) = mpsc::channel::<bool>();

        let task_join_handler = thread::spawn(move || {
            // let result = match task() {
            //     Ok(_) => tx.send(true),
            //     Err(_) => tx.send(false),
            // };
            //
            // if result.is_err() {
            //     return result.err();
            // }
            task();

            tx.send(true)
        });

        let _result = loop {
            if now.elapsed() >= Duration::from_millis(timeout) {
                return Err(CliPromptError::SpinnerError(SpinnerError::TimedOut));
            }

            self.term.write(
                format!(
                    "\r{} {}",
                    self.s_spinner_frames[spinner_symbol_index]
                        .clone()
                        .magenta(),
                    loading_message,
                )
                .as_bytes(),
            )?;

            self.term
                .write(format!("{}", ". ".repeat(spinner_symbol_index + 1)).as_bytes())?;
            self.term.clear_chars(2 * (spinner_symbol_index + 1))?;

            thread::sleep(Duration::from_millis(500));
            spinner_symbol_index = (spinner_symbol_index + 1) % 4;

            match rx.try_recv() {
                Ok(true) | Err(TryRecvError::Disconnected) => {
                    break true;
                }
                Ok(false) => {
                    break false;
                }
                Err(TryRecvError::Empty) => continue,
            }
        };
        // self.term.write_line("")?;
        self.term.clear_line()?;
        self.term
            .write_line(format!("{} {}", self.s_success.green(), finish_message).as_str())?;
        self.print_empty_line()?;

        match task_join_handler.join() {
            Ok(_) => {}
            Err(_e) => return Err(CliPromptError::SpinnerError(SpinnerError::TaskFailed)),
        }
        // if !result {
        //     return Err(CliPromptError::SpinnerError(SpinnerError::TaskFailed));
        // }
        Ok(())
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

    fn print_empty_line(&mut self) -> Result<()> {
        self.term.write_line(format!("{}", self.s_bar).as_str())
    }

    fn print_multi_options(
        &mut self,
        options: &Vec<PromptSelectOption>,
        is_selected: &Vec<bool>,
        current_choice: usize,
    ) -> Result<()> {
        let options_num = options.len();

        for i in 0..options_num {
            let current_option = &options.get(i).unwrap().label;

            self.term.write_line(&self.format_prefix(
                format!(
                    "{} {} {}",
                    if i == current_choice {
                        self.s_radio_active.green().to_string().clone()
                    } else {
                        self.s_radio_inactive.clone()
                    },
                    if is_selected[i] {
                        self.s_checkbox_active.blue().to_string().clone()
                    } else {
                        self.s_checkbox_inactive.clone()
                    },
                    current_option
                ),
                MessageType::Option,
            ))?;
        }

        self.print_multi_select_confirm(options_num, current_choice)
    }

    fn print_multi_select_confirm(
        &mut self,
        options_num: usize,
        current_choice: usize,
    ) -> Result<()> {
        self.term.write_line(&self.format_prefix(
            format!(
                "{} {}",
                if options_num == current_choice {
                    self.s_radio_active.green().to_string().clone()
                } else {
                    self.s_radio_inactive.clone()
                },
                "confirm"
            ),
            MessageType::Option,
        ))
    }

    #[allow(dead_code)]
    #[cfg(feature = "mock-term")]
    fn get_term_input(&self) -> Vec<u8> {
        self.term.get_input()
    }

    #[allow(dead_code)]
    #[cfg(feature = "mock-term")]
    fn get_term_output(&self) -> Vec<Vec<u8>> {
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
        self.term.clear_output();
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

/// Use to define options for [`CliPrompt::prompt_select`], [`CliPrompt::prompt_multi_select`]
#[derive(Debug, Clone, Eq, PartialEq)]
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

// #[cfg(feature = "mock-term")]
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    // #[cfg(feature = "unstable")]
    // use std::io::{Error, ErrorKind};

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
        prefix_map.insert("s_bar_h".to_owned(), get_symbol("─", "-", unicode_support));
        prefix_map.insert(
            "s_corner_top_right".to_owned(),
            get_symbol("╮", "+", unicode_support),
        );
        prefix_map.insert(
            "s_corner_bottom_right".to_owned(),
            get_symbol("╯", "+", unicode_support),
        );
        prefix_map.insert(
            "s_connect_left".to_owned(),
            get_symbol("├", "+", unicode_support),
        );
        prefix_map.insert(
            "s_checkbox_active".to_owned(),
            get_symbol("◼", "[+]", unicode_support),
        );
        prefix_map.insert(
            "s_checkbox_inactive".to_owned(),
            get_symbol("◻", "[ ]", unicode_support),
        );

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

        // let output = cli_prompt.get_term_output();
        let prefix_map = build_prefix_map();

        assert_eq!(
            format!(
                "{} message\n{}\n",
                prefix_map.get("s_bar_start").unwrap(),
                prefix_map.get("s_bar").unwrap()
            ),
            cli_prompt.term.get_output_string()
        );
    }

    #[test]
    fn test_outro() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.outro("message").unwrap();

        // let output = cli_prompt.get_term_output();
        let prefix_map = build_prefix_map();

        assert_eq!(
            format!("{} message\n", prefix_map.get("s_bar_end").unwrap()),
            cli_prompt.term.get_output_string()
        );
    }

    #[test]
    fn test_cancel() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.cancel("message").unwrap();

        // let output = cli_prompt.get_term_output();
        let prefix_map = build_prefix_map();

        assert_eq!(
            format!(
                "{} {}\n",
                prefix_map.get("s_bar_end").unwrap(),
                style("message").red()
            ),
            cli_prompt.term.get_output_string()
        );
    }

    #[test]
    fn test_log() {
        let prefix_map = build_prefix_map();

        let mut cli_prompt = CliPrompt::new();
        cli_prompt.log("message", LogType::Info).unwrap();

        // let mut output = cli_prompt.get_term_output();

        assert_eq!(
            format!(
                "{} {}\n{}\n",
                style(prefix_map.get("s_info").unwrap()).blue(),
                "message",
                prefix_map.get("s_bar").unwrap()
            ),
            cli_prompt.term.get_output_string()
        );

        cli_prompt.clear_term_output();
        cli_prompt.log("message", LogType::Warn).unwrap();
        // output = cli_prompt.get_term_output();

        assert_eq!(
            format!(
                "{} {}\n{}\n",
                style(prefix_map.get("s_warn").unwrap()).yellow(),
                style("message").yellow(),
                prefix_map.get("s_bar").unwrap()
            ),
            cli_prompt.term.get_output_string()
        );

        cli_prompt.clear_term_output();
        cli_prompt.log("message", LogType::Error).unwrap();
        // output = cli_prompt.get_term_output();

        assert_eq!(
            format!(
                "{} {}\n{}\n",
                style(prefix_map.get("s_error").unwrap()).red(),
                style("message").red(),
                prefix_map.get("s_bar").unwrap()
            ),
            cli_prompt.term.get_output_string()
        );
    }

    #[test]
    fn test_prompt_text() {
        let prefix_map = build_prefix_map();

        let mut cli_prompt = CliPrompt::new();

        cli_prompt.set_term_input("my name");

        let result = cli_prompt.prompt_text("name?").unwrap();

        assert_eq!(
            format!(
                "{} name?\n{} {}\n",
                style(prefix_map.get("s_step_submit").unwrap()).magenta(),
                prefix_map.get("s_bar").unwrap(),
                prefix_map.get("s_bar").unwrap()
            ),
            cli_prompt.term.get_output_string()
        );
        assert_eq!(result, "my name".to_string());
    }

    #[test]
    fn test_prompt_confirm_message() {
        let prefix_map = build_prefix_map();

        let mut cli_prompt = CliPrompt::new();

        cli_prompt.prompt_confirm("message").unwrap();

        assert_eq!(
            format!(
                "{} {}\n\r{} {} Yes / {} No\n{}\n",
                style(prefix_map.get("s_step_submit").unwrap()).magenta(),
                "message",
                prefix_map.get("s_bar").unwrap(),
                style(prefix_map.get("s_radio_active").unwrap()).green(),
                prefix_map.get("s_radio_inactive").unwrap(),
                prefix_map.get("s_bar").unwrap()
            ),
            cli_prompt.term.get_output_string()
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

        // let output = cli_prompt.get_term_output();

        assert_eq!(
            format!(
                "{} {}\n\
                \r{} {} {}\n\
                \r{} {} {}\n\
                {}\n",
                style(prefix_map.get("s_step_submit").unwrap()).magenta(),
                "message",
                prefix_map.get("s_bar").unwrap(),
                style(prefix_map.get("s_radio_active").unwrap()).green(),
                "test option 1",
                prefix_map.get("s_bar").unwrap(),
                prefix_map.get("s_radio_inactive").unwrap(),
                "test option 2",
                prefix_map.get("s_bar").unwrap()
            ),
            cli_prompt.term.get_output_string()
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

    #[test]
    fn test_prompt_select_empty_options() {
        let mut cli_prompt = CliPrompt::new();

        let options = vec![];

        let result = cli_prompt.prompt_select("message", options);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "options is empty");
    }

    #[test]
    fn test_print_note() {
        let prefix_map = build_prefix_map();
        let first_line = "hello";
        let second_line = "rust";

        let mut cli_prompt = CliPrompt::new();
        cli_prompt
            .print_note(format!("{}\n{}", first_line, second_line).as_str())
            .unwrap();

        assert_eq!(
            format!(
                "{}{}{}\n{} {} {}\n{} {}{}{}\n{}{}{}\n{}\n",
                prefix_map.get("s_connect_left").unwrap(),
                prefix_map
                    .get("s_bar_h")
                    .unwrap()
                    .repeat(first_line.len() + 2),
                prefix_map.get("s_corner_top_right").unwrap(),
                prefix_map.get("s_bar").unwrap(),
                first_line,
                prefix_map.get("s_bar").unwrap(),
                prefix_map.get("s_bar").unwrap(),
                second_line,
                " ".repeat(first_line.len() - second_line.len() + 1),
                prefix_map.get("s_bar").unwrap(),
                prefix_map.get("s_connect_left").unwrap(),
                prefix_map.get("s_bar_h").unwrap().repeat(7),
                prefix_map.get("s_corner_bottom_right").unwrap(),
                prefix_map.get("s_bar").unwrap(),
            ),
            cli_prompt.term.get_output_string()
        );
    }

    // FIXME: fix this after cursor mock done
    #[test]
    fn test_prompt_multi_select_message() {
        let prefix_map = build_prefix_map();
        let mut cli_prompt = CliPrompt::new();
        let options = vec![
            PromptSelectOption::new("option1", "test option 1"),
            PromptSelectOption::new("option2", "test option 2"),
        ];
        cli_prompt.push_key_input("arrow down");
        cli_prompt.push_key_input("arrow down");
        cli_prompt.prompt_multi_select("message", options).unwrap();

        assert_eq!(
            format!(
                "{} {}\n\
                \r{} {} {} {}\n\
                \r{} {} {} {}\n\
                \r{} {} {}\n\
                {}\n",
                style(prefix_map.get("s_step_submit").unwrap()).magenta(),
                "message",
                prefix_map.get("s_bar").unwrap(),
                style(prefix_map.get("s_radio_inactive").unwrap()).green(),
                prefix_map.get("s_checkbox_inactive").unwrap(),
                "test option 1",
                prefix_map.get("s_bar").unwrap(),
                prefix_map.get("s_radio_inactive").unwrap(),
                prefix_map.get("s_checkbox_inactive").unwrap(),
                "test option 2",
                prefix_map.get("s_bar").unwrap(),
                prefix_map.get("s_radio_active").unwrap(),
                "confirm",
                prefix_map.get("s_bar").unwrap()
            ),
            cli_prompt.term.get_output_string()
        );
    }

    #[test]
    fn test_prompt_multi_select_choose_none() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.push_key_input("arrow down");
        cli_prompt.push_key_input("arrow down");
        cli_prompt.push_key_input("enter");
        let options = vec![
            PromptSelectOption::new("option1", "test option 1"),
            PromptSelectOption::new("option2", "test option 2"),
        ];

        let selected_options = cli_prompt.prompt_multi_select("message", options).unwrap();

        assert_eq!(selected_options.len(), 0);
    }

    #[test]
    fn test_prompt_multi_select_choose_option1() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.push_key_input("enter");
        cli_prompt.push_key_input("arrow down");
        cli_prompt.push_key_input("arrow down");
        cli_prompt.push_key_input("enter");
        let options = vec![
            PromptSelectOption::new("option1", "test option 1"),
            PromptSelectOption::new("option2", "test option 2"),
        ];

        let selected_options = cli_prompt.prompt_multi_select("message", options).unwrap();

        assert_eq!(
            vec![PromptSelectOption::new("option1", "test option 1")],
            selected_options
        );
    }

    #[test]
    fn test_prompt_multi_select_choose_option1_and_option2() {
        let mut cli_prompt = CliPrompt::new();
        cli_prompt.push_key_input("enter");
        cli_prompt.push_key_input("arrow down");
        cli_prompt.push_key_input("enter");
        cli_prompt.push_key_input("arrow down");
        cli_prompt.push_key_input("enter");
        let options = vec![
            PromptSelectOption::new("option1", "test option 1"),
            PromptSelectOption::new("option2", "test option 2"),
        ];

        let selected_options = cli_prompt.prompt_multi_select("message", options).unwrap();

        assert_eq!(
            vec![
                PromptSelectOption::new("option1", "test option 1"),
                PromptSelectOption::new("option2", "test option 2"),
            ],
            selected_options
        );
    }

    #[test]
    fn test_test_prompt_multi_select_empty_options() {
        let mut cli_prompt = CliPrompt::new();

        let options = vec![];

        let result = cli_prompt.prompt_multi_select("message", options);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "options is empty");
    }

    #[test]
    fn test_prompt_multi_select_with_max_choice_num_empty_options() {
        let mut cli_prompt = CliPrompt::new();

        let options = vec![];

        let result = cli_prompt.prompt_multi_select_with_max_choice_num("message", options, 1);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "options is empty");
    }

    #[test]
    fn test_prompt_multi_select_with_max_choice_num_greater_than_options_length() {
        let mut cli_prompt = CliPrompt::new();

        let options = vec![
            PromptSelectOption::new("option1", "test option 1"),
            PromptSelectOption::new("option2", "test option 2"),
        ];

        let result = cli_prompt.prompt_multi_select_with_max_choice_num("message", options, 3);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error.to_string(),
            "max_choice_num must be less or equal than options length"
        );
    }

    #[test]
    fn test_prompt_multi_select_with_max_choice_num_zero() {
        let mut cli_prompt = CliPrompt::new();

        let options = vec![
            PromptSelectOption::new("option1", "test option 1"),
            PromptSelectOption::new("option2", "test option 2"),
        ];

        let result = cli_prompt.prompt_multi_select_with_max_choice_num("message", options, 0);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "max_choice_num must be greater than 0");
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn test_spinner_timeout() {
        let mut cli_prompt = CliPrompt::new();

        let pika = || -> Result<()> {
            thread::sleep(Duration::from_millis(5000));
            Ok(())
        };
        let result = cli_prompt.run_with_spinner("", "", 1000, pika).unwrap_err();

        assert_eq!("TimedOut".to_string(), result.to_string());
    }

    // #[test]
    // fn test_spinner_task_failed() {
    //     let mut cli_prompt = CliPrompt::new();
    //
    //     let pika = || -> std::io::Result<()> {
    //         thread::sleep(time::Duration::from_millis(1000));
    //         Err(Error::new(ErrorKind::Other, ""))
    //     };
    //     let result = cli_prompt.spinner_example("", 5000, pika).unwrap_err();
    //
    //     // assert_eq!(ErrorKind::Other, result.kind());
    //     assert_eq!("TaskFailed".to_string(), result.to_string());
    // }

    #[test]
    #[cfg(feature = "unstable")]
    fn test_spinner() {
        let mut cli_prompt = CliPrompt::new();

        let pika = || -> Result<()> {
            thread::sleep(Duration::from_millis(1000));
            Ok(())
        };
        let result = cli_prompt.run_with_spinner("", "", 5000, pika);

        assert!(result.is_ok());
    }
}
