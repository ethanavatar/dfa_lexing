
#[repr(C)]
#[derive(Debug, Clone)]
enum TokenKind {
    Number,
    Identifier,
    Operator,
    Delimiter,
}

#[repr(C)]
#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    value: String,
}

fn main() {
    let source = "let x = 10;";
    let tokens = lex(source);
    for token in tokens {
        dbg!(token);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Start,
    End,
    Number,
    Identifier,
    Operator,
    Delimiter,
    Whitespace,
}

struct Lexer {
    tokens: Vec<Token>,
    input: String,
    input_index: usize,

    state: State,
    token: String,
}

fn lex(source: &str) -> Vec<Token> {
    let mut lexer = Lexer {
        tokens: Vec::new(),
        input: source.to_string(),
        input_index: 0,

        state: State::Start,
        token: String::new(),
    };
    lexer.input.push('\0');

    while lexer.input_index < lexer.input.len() {
        let new_state = transition(&mut lexer);
        lexer.state = accept(&mut lexer, new_state);
    }

    return lexer.tokens.clone();
}

fn transition(lexer: &mut Lexer) -> State {
    let c = lexer.input.chars().nth(lexer.input_index).unwrap();
    let new_state = match c {
        '0'..='9' => State::Number,
        'a'..='z' | 'A'..='Z' => State::Identifier,
        '+' | '-' | '*' | '/' => State::Operator,
        '=' | ';' => State::Delimiter,
        ' ' | '\t' | '\n' => State::Whitespace,
        '\0' => State::End,
        _ => unimplemented!(),
    };
    return new_state;
}

fn accept(lexer: &mut Lexer, new_state: State) -> State {
    let c = lexer.input.chars().nth(lexer.input_index).unwrap();
    eprintln!("{}: {:?} -> {:?}", c, &lexer.state, new_state);

    match (&lexer.state, &new_state) {
        (State::Start, _) => {
            lexer.token.push(c);
            lexer.input_index += 1;
        },

        (State::Whitespace, State::Whitespace) => {
            lexer.input_index += 1;
        },

        (State::Whitespace, _) => { },

        (l, r) if l == r => {
            lexer.token.push(c);
            lexer.input_index += 1;
        },

        (l, r) if l != r => {
            lexer.tokens.push(Token {
                kind: match l {
                    State::Number => TokenKind::Number,
                    State::Identifier => TokenKind::Identifier,
                    State::Operator => TokenKind::Operator,
                    State::Delimiter => TokenKind::Delimiter,
                    _ => unimplemented!(),
                },
                value: lexer.token.clone(),
            });
            lexer.token.clear();
        },
        _ => unimplemented!(),
    }

    return new_state;
}

