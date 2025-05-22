pub struct Parser {
    // Fields for the parser can be defined here
    input: String
}

impl Parser {
    pub fn new(input: String) -> Self {
        // Initialize the parser
        Parser {
            input
        }
    }

    pub fn parse(&self) -> Result<AST, String> {
        let tokens: Vec<&str> = self.input.split_whitespace().collect();
        let mut iter = tokens.iter().peekable();

        let mut left = match iter.next() {
            Some(token) => AST::Number(token.parse::i32().map_err(|_| "Invalid number")?),
            None => return Err("Empty input".to_string()),
        };
    }
}