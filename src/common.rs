pub struct Lexer<'l> {
    line: &'l str,
    index: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(line: &'l str) -> Self {
        Self { line, index: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        self.line[self.index..].chars().next()
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
            
            Some(&self.line[range])
        } else {
            None
        }
    }
    
    pub fn rest(&self) -> &'l str {
        &self.line[self.index..]
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
        
        let text = &self.line[start..end];
        text.parse().ok()
    }
}
