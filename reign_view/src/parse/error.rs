use std::fmt;

// TODO:(view:err) Filename for error
pub struct Error {
    pub content: String,
    pub cursor: usize,
    pub message: String,
}

impl Error {
    fn get_line(&self) -> (usize, usize, String) {
        if self.cursor > self.content.len() {
            panic!(
                "error occurred at cursor {} which is over the content length {}",
                self.cursor,
                self.content.len()
            );
        }

        let lines: Vec<&str> = self.content.split('\n').collect();
        let mut cursor = self.cursor;
        let mut line_number = 0;
        let mut column_number = 0;

        for line in &lines {
            let line_length = line.len() + 1;

            if cursor >= line_length {
                cursor -= line_length;
                line_number += 1;
            } else {
                column_number = cursor;
                break;
            }
        }

        (
            column_number,
            line_number + 1,
            lines[line_number].to_string(),
        )
    }

    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO:(view:err) Make this colorful and prettier
        let info = self.get_line();
        let line_number = format!("{}", info.1);
        let start = format!("{:>1$}", "|", line_number.len() + 2);

        let column_spaces = if info.0 == 0 {
            "".to_string()
        } else {
            format!("{:<1$}", " ", info.0)
        };

        writeln!(f)?;
        writeln!(f, "{}", start)?;
        writeln!(f, "{} | {}", line_number, info.2)?;
        writeln!(
            f,
            "{} {}{:^<3$}",
            start,
            column_spaces,
            "-",
            info.2.len() - info.0 + 1
        )?;
        writeln!(f, "{} {:>2$}", start, "|", info.0 + 1)?;
        writeln!(f, "{} {}{}", start, column_spaces, self.message)?;
        writeln!(f, "{}", start)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(f)
    }
}

#[cfg(test)]
mod test {
    use super::super::ParseStream;

    #[test]
    fn test_fmt_start() {
        let mut ps = ParseStream::new("Hello".to_string());
        let err = ps.step("W").unwrap_err();

        assert_eq!(
            format!("{:?}", err),
            "
  |
1 | Hello
  | -^^^^^
  | |
  | expected `W`
  |
"
        );
    }

    #[test]
    fn test_fmt_middle() {
        let mut ps = ParseStream::new("Hello".to_string());
        ps.step("He").unwrap();
        let err = ps.step("W").unwrap_err();

        assert_eq!(
            format!("{:?}", err),
            "
  |
1 | Hello
  |   -^^^
  |   |
  |   expected `W`
  |
"
        );
    }

    #[test]
    fn test_fmt_newline() {
        let mut ps = ParseStream::new("Hello\nWorld".to_string());
        ps.step("Hello").unwrap();
        let err = ps.step("W").unwrap_err();

        assert_eq!(
            format!("{:?}", err),
            "
  |
1 | Hello
  |      -
  |      |
  |      expected `W`
  |
"
        );
    }

    #[test]
    fn test_fmt_next_line_start() {
        let mut ps = ParseStream::new("Hello\nWorld".to_string());
        ps.step("Hello").unwrap();
        ps.skip_spaces().unwrap();
        let err = ps.step("or").unwrap_err();

        assert_eq!(
            format!("{:?}", err),
            "
  |
2 | World
  | -^^^^^
  | |
  | expected `or`
  |
"
        );
    }

    #[test]
    fn test_fmt_next_line_middle() {
        let mut ps = ParseStream::new("Hello\nWorld".to_string());
        ps.step("Hello").unwrap();
        ps.skip_spaces().unwrap();
        ps.step("Wor").unwrap();
        let err = ps.step("or").unwrap_err();

        assert_eq!(
            format!("{:?}", err),
            "
  |
2 | World
  |    -^^
  |    |
  |    expected `or`
  |
"
        );
    }

    #[test]
    fn test_fmt_next_line_newline() {
        let mut ps = ParseStream::new("Hello\nWorld\n".to_string());
        ps.step("Hello").unwrap();
        ps.skip_spaces().unwrap();
        ps.step("World").unwrap();
        let err = ps.step("!").unwrap_err();

        assert_eq!(
            format!("{:?}", err),
            "
  |
2 | World
  |      -
  |      |
  |      expected `!`
  |
"
        );
    }

    #[test]
    fn test_fmt_eof() {
        let mut ps = ParseStream::new("Hello".to_string());
        ps.step("Hello").unwrap();
        let err = ps.step("!").unwrap_err();

        assert_eq!(
            format!("{:?}", err),
            "
  |
1 | Hello
  |      -
  |      |
  |      out of bounds when trying to find `!`
  |
"
        );
    }
}
