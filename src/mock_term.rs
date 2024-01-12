pub(crate) mod mock_term {
    use crate::mock_term::mock_term;
    use std::collections::VecDeque;
    use std::io::Write;

    pub struct Term {
        cursor_hidden: bool,
        current_cursor: usize,
        pub input: Vec<u8>,
        pub output: Vec<u8>,
        pub key_input: VecDeque<String>,
    }

    #[allow(dead_code)]
    impl Term {
        pub fn stdout() -> Self {
            Self {
                cursor_hidden: true,
                current_cursor: 0,
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

        pub fn is_cursor_hidden(&self) -> bool {
            self.cursor_hidden
        }

        pub fn get_current_cursor(&self) -> usize {
            self.current_cursor
        }

        pub fn get_input(&self) -> Vec<u8> {
            self.input.clone()
        }

        pub fn get_output(&self) -> Vec<u8> {
            self.output.clone()
        }

        pub fn show_cursor(&mut self) -> Result<(), std::io::Error> {
            self.cursor_hidden = true;
            Ok(())
        }

        pub fn hide_cursor(&mut self) -> Result<(), std::io::Error> {
            self.cursor_hidden = false;
            Ok(())
        }

        pub fn read_key(&mut self) -> Result<mock_term::Key, std::io::Error> {
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

        // TODO: need mock implementation for testing message
        pub fn move_cursor_down(&self, _: usize) -> Result<(), std::io::Error> {
            Ok(())
        }

        // TODO: need mock implementation for testing message
        pub fn move_cursor_up(&self, _: usize) -> Result<(), std::io::Error> {
            Ok(())
        }

        pub fn clear_chars(&self, _: usize) -> Result<(), std::io::Error> {
            Ok(())
        }

        pub fn clear_line(&self) -> Result<(), std::io::Error> {
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

#[cfg(test)]
mod tests {
    use crate::mock_term::mock_term::Term;

    #[test]
    fn test_cursor() {
        let mut pika = Term::stdout();

        assert!(pika.is_cursor_hidden());

        pika.hide_cursor().unwrap();

        assert_eq!(pika.is_cursor_hidden(), false);

        pika.show_cursor().unwrap();

        assert_eq!(pika.is_cursor_hidden(), true);
    }

}