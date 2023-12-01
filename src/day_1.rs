
pub fn puzzle_1(input: &str) -> usize {
    let mut result = 0;

    let mut digits = Vec::new();

    for line in input.lines() {
        digits.clear();
        for c in line.chars() {
            if c.is_ascii_digit() {
                digits.push((c as u8) - b'0');
            }
        }

        let mut num = 0;

        if let Some(&first) = digits.first() {
            num = first * 10;
        }

        if let Some(&last) = digits.last() {
            num += last;
        }

        result += num as usize;
    }

    result
}

pub fn puzzle_2(input: &str) -> usize {
    let mut result = 0;

    let mut digits: Vec<u8> = Vec::new();

    const DIGITS: &[(&str, u8)] = &[
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    struct State<'l> {
        line: &'l str,
        index: usize,
    }

    impl<'l> State<'l> {
        fn new(line: &'l str) -> Self {
            Self {
                line,
                index: 0,
            }
        }

        fn peek(&self) -> Option<char> {
            self.line[self.index..].chars().next()
        }

        fn advance(&mut self) -> Option<char> {
            let c = self.peek()?;
            self.index += c.len_utf8();
            Some(c)
        }

        fn next_digit(&mut self) -> Option<u8> {
            loop {
                let c = self.peek()?;

                break Some(match c {
                    '0'..='9' => {
                        self.advance();
                        (c as u8) - b'0'
                    },
                    _ => {
                        let rest = &self.line[self.index..];
                        
                        for (digit, value) in DIGITS.iter().copied() {
                            if rest.starts_with(digit) {
                                self.index += 1;
                                return Some(value);
                            }
                        }

                        self.advance();
                        continue;
                    }
                })
            }
        }
    }

    for line in input.lines() {
        digits.clear();
        let mut state = State::new(line);

        while let Some(digit) = state.next_digit() {
            digits.push(digit);
        }

        let mut num = 0;

        if let Some(&first) = digits.first() {
            num = first * 10;
        }

        if let Some(&last) = digits.last() {
            num += last;
        }

        result += num as usize;
    }

    result
}