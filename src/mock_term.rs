pub(crate) mod mock_term {
    use crate::mock_term::mock_term;
    use std::collections::VecDeque;
    use std::io::Write;

    pub struct Term {
        cursor_hidden: bool,
        current_cursor: (usize, usize),
        pub input: Vec<u8>,
        pub output: Vec<Vec<u8>>,
        pub key_input: VecDeque<String>,
    }

    #[allow(dead_code)]
    impl Term {
        pub(crate) fn stdout_with_output_and_cursor(
            initial_output: Vec<Vec<u8>>,
            initial_cursor: (usize, usize),
        ) -> Self {
            Self {
                cursor_hidden: true,
                current_cursor: initial_cursor,
                input: vec![],
                output: initial_output,
                key_input: VecDeque::new(),
            }
        }

        pub fn stdout() -> Self {
            Self {
                cursor_hidden: true,
                current_cursor: (0, 0),
                input: vec![],
                output: vec![],
                key_input: VecDeque::new(),
            }
        }

        pub fn write_line(&mut self, s: &str) -> Result<(), std::io::Error> {
            self.write(s.as_bytes())?;
            self.current_cursor.0 += 1;
            self.current_cursor.1 = 0;

            Ok(())
        }

        pub fn is_cursor_hidden(&self) -> bool {
            self.cursor_hidden
        }

        pub fn get_current_cursor(&self) -> (usize, usize) {
            self.current_cursor
        }

        pub fn get_input(&self) -> Vec<u8> {
            self.input.clone()
        }

        pub fn get_output(&self) -> Vec<Vec<u8>> {
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
        pub fn move_cursor_down(&mut self, n: usize) -> Result<(), std::io::Error> {
            self.current_cursor.0 = if self.output.len() >= n + self.current_cursor.0 {
                self.current_cursor.0 + n
            } else {
                self.output.len()
            };

            Ok(())
        }

        // TODO: need mock implementation for testing message
        pub fn move_cursor_up(&mut self, n: usize) -> Result<(), std::io::Error> {
            self.current_cursor.0 = if self.current_cursor.0 >= n {
                self.current_cursor.0 - n
            } else {
                0
            };

            Ok(())
        }

        pub fn clear_chars(&self, _: usize) -> Result<(), std::io::Error> {
            Ok(())
        }

        pub fn clear_line(&self) -> Result<(), std::io::Error> {
            Ok(())
        }

        pub fn get_output_string(&self) -> String {
            self.output
                .iter()
                .map(|line| String::from_utf8_lossy(line).into_owned())
                .chain(if self.output.len() == self.current_cursor.0 {
                    Some(String::new())
                } else {
                    None
                })
                .collect::<Vec<String>>()
                .join("\n")
        }

        pub fn clear_output(&mut self) {
            self.output.clear();
            self.current_cursor = (0, 0);
        }
    }

    impl Write for Term {
        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            let buf_str = std::str::from_utf8(buf).unwrap();
            let buf_str_split_vec: Vec<&str> = buf_str.lines().collect();

            for (index, split_str) in buf_str_split_vec.iter().enumerate() {
                match self.output.get_mut(self.current_cursor.0) {
                    Some(current_line_output) => {
                        if split_str.starts_with("\r") {
                            self.current_cursor.1 = 0;
                        }

                        let updated_line_output: Vec<u8> =
                            if current_line_output.len() <= self.current_cursor.1 {
                                let padding =
                                    " ".repeat(self.current_cursor.1 - current_line_output.len());
                                current_line_output.extend(padding.as_bytes());
                                current_line_output.extend_from_slice(split_str.as_bytes());
                                current_line_output.to_owned()
                            } else {
                                let split_str_bytes: Vec<u8> = split_str.as_bytes().to_vec();

                                for (i, &byte) in split_str_bytes.iter().enumerate() {
                                    if self.current_cursor.1 + i < current_line_output.len() {
                                        current_line_output[self.current_cursor.1 + i] = byte;
                                    } else {
                                        current_line_output.push(byte);
                                    }
                                }
                                current_line_output.to_owned()
                            };
                        self.output[self.current_cursor.0] = updated_line_output;
                    }
                    None => self.output.push(split_str.as_bytes().to_vec()),
                }

                if buf_str_split_vec.len() > index + 1 {
                    self.current_cursor.0 = self.current_cursor.0 + 1;
                    self.current_cursor.1 = 0;
                } else {
                    // cursor remain
                    self.current_cursor.1 = split_str.len();
                }
            }

            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    #[allow(dead_code)]
    #[derive(PartialEq, Eq, Hash, Debug)]
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
    use crate::mock_term::mock_term::{Term, Key};
    use std::io::Write;

    #[test]
    fn test_initial_output() {
        let initial_output: Vec<Vec<u8>> = vec![b"qweqwe".to_vec(), b"qqq".to_vec()];
        let mock_term = Term::stdout_with_output_and_cursor(initial_output, (0, 0));

        assert_eq!(
            mock_term.get_output(),
            vec![b"qweqwe".to_vec(), b"qqq".to_vec(),]
        );
    }

    #[test]
    fn test_initial_output_and_cursor() {
        let initial_output: Vec<Vec<u8>> = vec![b"qweqwe".to_vec(), b"qqq".to_vec()];
        let mock_term = Term::stdout_with_output_and_cursor(initial_output, (0, 0));

        assert_eq!(mock_term.get_current_cursor(), (0,0));
        assert_eq!(
            mock_term.get_output(),
            vec![b"qweqwe".to_vec(), b"qqq".to_vec(),]
        );
        assert_eq!(mock_term.get_current_cursor(), (0,0));
    }

    #[test]
    fn test_clear_output() {
        let initial_output: Vec<Vec<u8>> = vec![b"qweqwe".to_vec(), b"qqq".to_vec()];
        let mut mock_term = Term::stdout_with_output_and_cursor(initial_output, (0, 0));
        mock_term.clear_output();

        assert!(mock_term.get_output().is_empty());
        assert_eq!(mock_term.get_current_cursor(), (0, 0));
    }

    #[test]
    fn test_write() {
        let mut mock_term = Term::stdout();

        mock_term.write(b"hahaha").unwrap();

        assert_eq!(mock_term.get_output_string(), "hahaha");
        assert_eq!(mock_term.get_current_cursor(), (0, 6));
    }

    #[test]
    fn test_write_with_initial_output() {
        let initial_output: Vec<Vec<u8>> = vec![b"qweqwe".to_vec(), b"qqq".to_vec()];
        let mut mock_term = Term::stdout_with_output_and_cursor(initial_output, (2, 0));

        mock_term.write(b"hello\nthere").unwrap();

        assert_eq!(mock_term.get_output_string(), "qweqwe\nqqq\nhello\nthere");
        assert_eq!(mock_term.get_current_cursor(), (3, 5));
    }

    #[test]
    fn test_write_with_initial_output_and_cursor() {
        let initial_output: Vec<Vec<u8>> = vec![b"qweqwe".to_vec(), b"qqq".to_vec()];
        let mut mock_term = Term::stdout_with_output_and_cursor(initial_output, (1, 3));

        mock_term.write(b"hello\nthere").unwrap();

        assert_eq!(mock_term.get_output_string(), "qweqwe\nqqqhello\nthere");
        assert_eq!(mock_term.get_current_cursor(), (2, 5));
    }

    #[test]
    fn test_write_with_initial_output_and_cursor_when_new_output_has_cr() {
        let initial_output: Vec<Vec<u8>> = vec![b"qweqwe".to_vec(), b"qqq".to_vec()];
        let mut mock_term = Term::stdout_with_output_and_cursor(initial_output, (1, 3));

        mock_term.write(b"\rhello\nthere").unwrap();

        assert_eq!(mock_term.get_output_string(), "qweqwe\n\rhello\nthere");
        assert_eq!(mock_term.get_current_cursor(), (2, 5));
    }

    #[test]
    fn test_write_with_initial_output_and_cursor_when_new_output_has_cr_and_initial_output_is_shorter_than_new_output(
    ) {
        let initial_output: Vec<Vec<u8>> = vec![b"qweqwe".to_vec(), b"qqqxxxxxx".to_vec()];
        let mut mock_term = Term::stdout_with_output_and_cursor(initial_output, (1, 9));

        mock_term.write(b"\rhello\nthere").unwrap();

        assert_eq!(mock_term.get_output_string(), "qweqwe\n\rhelloxxx\nthere");
        assert_eq!(mock_term.get_current_cursor(), (2, 5));
    }

    #[test]
    fn test_write_line() {
        let mut mock_term = Term::stdout();

        assert_eq!(mock_term.get_output_string(), "");

        mock_term.write_line("aaaa").unwrap();

        assert_eq!(mock_term.get_output_string(), "aaaa\n");
        assert_eq!(mock_term.get_current_cursor(), (1, 0));
    }

    #[test]
    fn test_cursor_hidden() {
        let mut mock_term = Term::stdout();

        assert!(mock_term.is_cursor_hidden());

        mock_term.hide_cursor().unwrap();

        assert_eq!(mock_term.is_cursor_hidden(), false);

        mock_term.show_cursor().unwrap();

        assert_eq!(mock_term.is_cursor_hidden(), true);
    }

    #[test]
    fn test_move_cursor() {
        let initial_output: Vec<Vec<u8>> = vec![b"qweqwe".to_vec(), b"qqqxxxxxx".to_vec()];
        let mut mock_term = Term::stdout_with_output_and_cursor(initial_output, (1, 9));

        mock_term.move_cursor_up(4).unwrap();

        assert_eq!(mock_term.get_current_cursor(), (0, 9));

        mock_term.move_cursor_down(10).unwrap();

        assert_eq!(mock_term.get_current_cursor(), (2, 9));

        mock_term.move_cursor_up(2).unwrap();
        mock_term.write(b"aaa\naa").unwrap();

        assert_eq!(mock_term.get_output_string(), "qweqwe   aaa\naaqxxxxxx");
        assert_eq!(mock_term.get_current_cursor(), (1, 2));
    }

    #[test]
    fn test_get_output() {
        let initial_output: Vec<Vec<u8>> = vec![b"qweqwe".to_vec(), b"qqqxxxxxx".to_vec()];
        let mock_term = Term::stdout_with_output_and_cursor(initial_output, (1, 9));

        let output = mock_term.get_output();

        assert_eq!(output, vec![b"qweqwe".to_vec(), b"qqqxxxxxx".to_vec()]);
    }

    #[test]
    fn test_flush() {
        let mut mock_term = Term::stdout();

        assert!(mock_term.flush().is_ok());
    }

    #[test]
    fn test_get_output_string_empty_output() {
        let mock_term = Term::stdout();

        assert_eq!(mock_term.get_output_string(), "");
    }

    #[test]
    fn test_read_line() {
        let mut mock_term = Term::stdout();
        mock_term.input = b"asdfasdf".to_vec();

        let read_result = mock_term.read_line();

        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), "asdfasdf");
    }

    #[test]
    fn test_read_key_empty_input() {
        let mut mock_term = Term::stdout();
        
        assert!(mock_term.key_input.is_empty());

        let read_key_result = mock_term.read_key();

        assert!(read_key_result.is_ok());
        assert_eq!(read_key_result.unwrap(), Key::Enter);
    }

    #[test]
    fn test_read_key() {
        let mut mock_term = Term::stdout();

        mock_term.key_input.push_back("arrow left".to_string());
        mock_term.key_input.push_back("arrow right".to_string());
        mock_term.key_input.push_back("arrow down".to_string());
        mock_term.key_input.push_back("arrow up".to_string());
        mock_term.key_input.push_back("enter".to_string());
        mock_term.key_input.push_back("asmqwelmasmd".to_string());

        let mut read_key_result_vec = vec![];

        for _ in 0..6 {
            read_key_result_vec.push(mock_term.read_key().unwrap());
        }

        assert_eq!(read_key_result_vec, vec![
            Key::ArrowLeft,
            Key::ArrowRight,
            Key::ArrowDown,
            Key::ArrowUp,
            Key::Enter,
            Key::Unknown,
        ]);
    }

    #[test]
    fn test_clear_chars() {
        let mock_term = Term::stdout();

        assert!(mock_term.clear_chars(0).is_ok());
    }

    #[test]
    fn test_clear_line() {
        let mock_term = Term::stdout();

        assert!(mock_term.clear_line().is_ok());
    }
}
