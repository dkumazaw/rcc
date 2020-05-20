use std::collections::linked_list::Iter;
use std::collections::LinkedList;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    TKRESERVED,
    TKNUM,
    TKEOF,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    string: Option<String>,
    val: i32,
}

pub struct Tokenizer {
    tokens: LinkedList<Token>, // Linked list of tokens
}

pub struct TokenIter<'a> {
    iter: Peekable<Iter<'a, Token>>,
}

impl Token {
    fn new(kind: TokenKind) -> Self {
        // TODO: Can we have some default behavior?
        Token {
            kind: kind,
            string: None,
            val: 0,
        }
    }

    fn val(mut self, value: i32) -> Self {
        self.val = value;
        self
    }

    fn string(mut self, s: &str) -> Self {
        self.string = Some(String::from(s));
        self
    }
}

impl Tokenizer {
    // Constructor
    pub fn new() -> Self {
        Tokenizer {
            tokens: LinkedList::new(),
        }
    }

    // Tokenizes the passed str
    pub fn tokenize(&mut self, in_str: &str) -> Iter<Token> {
        use TokenKind::*;

        let len = in_str.len();
        let mut cur = 0;

        while cur != len {
            let c = in_str.chars().nth(cur).unwrap();
            match c {
                c if c.is_whitespace() => {
                    cur += 1;
                    continue;
                }

                '+' | '-' | '*' | '/' | '(' | ')' => {
                    self.tokens
                        .push_back(Token::new(TKRESERVED).string(&c.to_string()));
                    cur += 1;
                    continue;
                }

                c if c.is_numeric() => {
                    let mut val = c.to_digit(10).unwrap() as i32;
                    cur += 1;
                    while cur != len {
                        let _c = in_str.chars().nth(cur).unwrap();
                        if !_c.is_numeric() {
                            break;
                        }
                        val = val * 10 + (_c.to_digit(10).unwrap() as i32);
                        cur += 1;
                    }
                    self.tokens.push_back(Token::new(TKNUM).val(val));
                    continue;
                }

                _ => panic!("Unexpected char."),
            }
        }

        // Finally add tof
        self.tokens.push_back(Token::new(TKEOF));

        self.tokens.iter()
    }
}

impl<'a> TokenIter<'a> {
    pub fn new(baseiter: Iter<'a, Token>) -> Self {
        TokenIter {
            iter: baseiter.peekable(),
        }
    }

    pub fn expect(&mut self, s: &str) {
        let t = self.next();
        if t.kind != TokenKind::TKRESERVED {
            panic!("TokenIter: Expected reserved token {}", s);
        }
        if let Some(ref tkstr) = t.string {
            if tkstr != s {
                panic!("TokenIter: Wrong token string.")
            }
        } else {
            panic!("TokenIter: Expected that string exists.")
        }
    }

    pub fn expect_number(&mut self) -> i32 {
        let t = self.next();
        if t.kind != TokenKind::TKNUM {
            panic!("TokenIter: Expected number.")
        }

        t.val
    }

    pub fn consume(&mut self, s: &str) -> bool {
        let t = self.peek();
        if t.kind == TokenKind::TKRESERVED {
            if let Some(ref tkstr) = t.string {
                if tkstr == s {
                    self.next();
                    return true;
                }
            }
        }
        false
    }

    pub fn at_eof(&mut self) -> bool {
        self.peek().kind == TokenKind::TKEOF
    }

    // Wrapper to hide option unwrapping
    fn peek(&mut self) -> &Token {
        self.iter.peek().unwrap()
    }

    // Wrapper to hide option unwrapping
    fn next(&mut self) -> &Token {
        self.iter.next().unwrap()
    }
}
