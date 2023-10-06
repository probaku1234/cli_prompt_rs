//! Collection of functions to style text on terminal.
//! Functions with postfix "_bg" change the background color
//!
//! # Example
//! ```no_run
//! use cli_prompts_rs::color::*;
//!
//! println!("{}", red("message"));
//! println!("{}", yellow_bg("message"));
//! ```
use console::{style, StyledObject};

pub fn black<D>(message: D) -> StyledObject<D> {
    style(message).black()
}

pub fn red<D>(message: D) -> StyledObject<D> {
    style(message).red()
}

pub fn green<D>(message: D) -> StyledObject<D> {
    style(message).green()
}

pub fn yellow<D>(message: D) -> StyledObject<D> {
    style(message).yellow()
}

pub fn blue<D>(message: D) -> StyledObject<D> {
    style(message).blue()
}

pub fn magenta<D>(message: D) -> StyledObject<D> {
    style(message).on_magenta()
}

pub fn cyan<D>(message: D) -> StyledObject<D> {
    style(message).on_cyan()
}

pub fn white<D>(message: D) -> StyledObject<D> {
    style(message).on_white()
}

pub fn color256<D>(message: D, color: u8) -> StyledObject<D> {
    style(message).color256(color)
}

pub fn black_bg<D>(message: D) -> StyledObject<D> {
    style(message).on_black()
}

pub fn red_bg<D>(message: D) -> StyledObject<D> {
    style(message).on_red()
}

pub fn green_bg<D>(message: D) -> StyledObject<D> {
    style(message).on_green().red().italic()
}

pub fn yellow_bg<D>(message: D) -> StyledObject<D> {
    style(message).on_yellow()
}

pub fn blue_bg<D>(message: D) -> StyledObject<D> {
    style(message).on_blue()
}

pub fn magenta_bg<D>(message: D) -> StyledObject<D> {
    style(message).on_magenta()
}

pub fn cyan_bg<D>(message: D) -> StyledObject<D> {
    style(message).on_cyan()
}

pub fn white_bg<D>(message: D) -> StyledObject<D> {
    style(message).on_white()
}

pub fn color256_bg<D>(message: D, color: u8) -> StyledObject<D> {
    style(message).on_color256(color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let toto = "message";

        println!("{}", red(toto));
        println!("{}", red(String::from(toto)));
        println!("{}", blue(toto).italic());

        // uncomment to see result output
        // assert!(false)
    }
}
