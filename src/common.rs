pub struct Lexer<'l> {
    source: &'l str,
    index: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Self {
        Self { source, index: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.index..].chars().next()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.index += c.len_utf8();
        Some(c)
    }

    #[must_use]
    pub fn advance_matches(&mut self, text: &str) -> Option<&'l str> {
        if self.rest().starts_with(text) {
            let start = self.index;
            self.index += text.len();
            let range = start..self.index;

            Some(&self.source[range])
        } else {
            None
        }
    }

    pub fn rest(&self) -> &'l str {
        &self.source[self.index..]
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn parse_word(&mut self) -> Option<&'l str> {
        let start = self.index;

        while let Some(c) = self.peek() {
            if c.is_ascii_alphabetic() {
                self.advance();
            } else {
                break;
            }
        }

        let end = self.index;

        if start == end {
            return None;
        }

        Some(&self.source[start..end])
    }

    pub fn parse_signed_int(&mut self) -> Option<isize> {
        let c = self.peek()?;

        let sign = match c {
            '-' => {
                self.advance();
                -1
            },
            '+' => {
                self.advance();
                1
            }
            _ => 1
        };

        let unsigned = self.parse_int()?;

        Some((unsigned as isize) * sign)
    }

    pub fn parse_int(&mut self) -> Option<usize> {
        let start = self.index;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let end = self.index;
        if start == end {
            return None;
        }

        let text = &self.source[start..end];
        text.parse().ok()
    }
}

#[derive(Clone, Copy)]
pub struct Grid<'s> {
    source: &'s [u8],
    width: usize,
    height: usize,
}

impl<'s> Grid<'s> {
    pub fn new(source: &'s [u8], width: usize) -> Self {
        let height = source.len() / width;
        Self {
            source,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.width && y < self.height, "Address out of bounds");

        let index = (y * self.width) + x;
        self.source[index]
    }
}
