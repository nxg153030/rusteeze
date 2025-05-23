pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Option<String> {
        // Skip whitespace
        while self.position < self.input.len() && self.input[self.position..].starts_with(char::is_whitespace) {
            self.position += 1;
        }
        // Check if we reached the end of the input
        if self.position >= self.input.len() {
            return None;
        }
        
        // Extract the next token
        let current_char = self.input.chars().nth(self.position).unwrap();

        if current_char.is_digit(10) {
            // Extract a number
            let start = self.position;
            while self.position < self.input.len() && self.input[self.position..].starts_with(char::is_digit) {
                self.position += 1;
            }
            return Some(self.input[start..self.position].to_string());
        } else if current_char.is_alphabetic() {
            // Extract an identifier
            let start = self.position;
            while self.position < self.input.len() && self.input[self.position..].starts_with(char::is_alphanumeric) {
                self.position += 1;
            }
            return Some(self.input[start..self.position].to_string());
        } else {
            // Handle single-character token (e.g., operator or punctuation)
            self.position += 1;
            return Some(current_char.to_string());
        }

    }
}