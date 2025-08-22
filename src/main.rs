pub enum Token {
    Word(String),
    Option(String),
    Pipe,           // |
    And,            // &&
    Or,             // ||
    Semicolon,      // ;
    Background,     // &
    RedirectIn,     // <
    RedirectOut,    // >
    RedirectAppend, // >>
    Heredoc,        // <<
    Assignment { var: String, value: String },
    Variable(String),
    CommandSubstitutionStart, // $(
    CommandSubstitutionEnd,   // )
    SingleQuoteStart,         // '
    SingleQuoteEnd,           // '
    DoubleQuoteStart,         // "
    DoubleQuoteEnd,           // "
    Comment(String),
    Newline,
}

enum Status {
    Quotes(char),
    Word()
    Operation(char),
    Null,
}

struct Tokens {
    input: String(),
    tokens: Vec<Token>,
}

impl Tokens {
    fn new(input: &str) -> Tokens {
        let mut tkns = Tokens {
            input: input.to_string(),
            tokens: Vec::new(),
        };
        let mut status: Status = Status::Null;
        let chars: Vec<char> = input.chars.collect::<Vec<char>>();
        let mut index: usize = 0;
        let mut word: String = String::new();
        while index < input.len() {
            match chars[index] {
                "\"" | "'" => {
                    if status == Status::Word(chars[index]) {
                    } else if status == Status::Null {
                    } else {
                    }
                }
            }
        }
    }
}

fn main() {}
