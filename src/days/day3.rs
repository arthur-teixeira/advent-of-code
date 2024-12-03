#[derive(Debug, PartialEq, Eq)]
enum Token {
    MUL,
    INT(usize),
    COMMA,
    LPAREN,
    RPAREN,
    GARBAGE,
    EOF,
}

impl Token {
    fn as_num(&self) -> Option<usize> {
        match self {
            Self::INT(a) => Some(*a),
            _ => None,
        }
    }
}

struct Lexer {
    text: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Self {
            text: input.chars().collect(),
            pos: 0,
        }
    }

    fn get_mul(&mut self) -> Option<Token> {
        let buf = &self.text[self.pos..self.pos + 3];
        let token = match buf {
            ['m', 'u', 'l'] => Some(Token::MUL),
            _ => None,
        };

        self.pos += 2;
        token
    }

    fn get_num(&mut self) -> Token {
        let start = self.pos;
        while self.text[self.pos + 1].is_ascii_digit() {
            self.pos += 1;
            if self.pos >= self.text.len() - 1 {
                return Token::EOF;
            }
        }

        let num = self.text[start..=self.pos]
            .iter()
            .cloned()
            .collect::<String>();
        let parsed = num
            .parse::<usize>()
            .expect("Should have collected only digits");

        Token::INT(parsed)
    }

    fn next_token(&mut self) -> Token {
        loop {
            if self.pos >= self.text.len() {
                return Token::EOF;
            }

            let cur = match self.text[self.pos] {
                'm' => self.get_mul(),
                '(' => Some(Token::LPAREN),
                ')' => Some(Token::RPAREN),
                ',' => Some(Token::COMMA),
                d if d.is_ascii_digit() => Some(self.get_num()),
                _ => Some(Token::GARBAGE),
            };

            self.pos += 1;

            match cur {
                Some(tok) => return tok,
                None => continue,
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::EOF => None,
            other => Some(other),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Mul(usize, usize);

impl Mul {
    fn eval(&self) -> usize {
        self.0 * self.1
    }
}

struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(input: String) -> Self {
        let mut lex = Lexer::new(input);
        let cur_token = lex.next_token();
        let peek_token = lex.next_token();

        Self {
            l: lex,
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.l.next_token());
    }

    fn expect_next_token(&mut self, t: Token) -> Option<()> {
        match t {
            Token::INT(_) => {
                if let Token::INT(_) = self.peek_token {
                    self.next_token();
                    Some(())
                } else {
                    None
                }
            }
            t => {
                if self.peek_token == t {
                    self.next_token();
                    Some(())
                } else {
                    None
                }
            }
        }
    }

    fn parse_mul(&mut self) -> Option<Mul> {
        // we start at Token::MUL
        self.expect_next_token(Token::LPAREN)?;
        self.expect_next_token(Token::INT(0))?;
        let left = self.cur_token.as_num().expect("Expected int");
        self.expect_next_token(Token::COMMA);
        self.expect_next_token(Token::INT(0))?;
        let right = self.cur_token.as_num().expect("Expected int");
        self.expect_next_token(Token::RPAREN)?;

        Some(Mul(left, right))
    }

    fn next_expression(&mut self) -> Option<Mul> {
        loop {
            let cur = match self.cur_token {
                Token::MUL => self.parse_mul(),
                Token::EOF => return None,
                _ => None,
            };

            if let Some(mul) = cur {
                return Some(mul);
            }
            self.next_token();
        }
    }
}

impl Iterator for Parser {
    type Item = Mul;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_expression()
    }
}

pub fn day3(input: String) {
    println!("Part 1: {}", part1(input));
}

fn part1(input: String) -> usize {
    let parser = Parser::new(input);
    parser.into_iter()
        .map(|cur| {
            println!("{:?}", cur);
            cur
        })
        .fold(0, |acc, cur| acc + cur.eval())
}

#[cfg(test)]
mod day3_test {
    use super::{Mul, Parser};

    fn parse(input: &str) -> Vec<Mul> {
        let parser = Parser::new(input.into());
        parser.into_iter().collect()
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse("mul(300,5)"), vec![Mul(300, 5)]);
        assert_eq!(
            parse("mul(300,5) mul(10,10)"),
            vec![Mul(300, 5), Mul(10, 10)]
        );
        assert_eq!(
            parse("mul(300,5)$%$%$%$%$mul(10,10)"),
            vec![Mul(300, 5), Mul(10, 10)]
        );
        assert_eq!(
            parse("mul(300,5)mulmul(10,10)"),
            vec![Mul(300, 5), Mul(10, 10)]
        );
        assert_eq!(
            parse("mul(300,5)mul(10,5]mul(10,10)"),
            vec![Mul(300, 5), Mul(10, 10)]
        );
        assert_eq!(
            parse("mul(300,5)mul(10%5]mmumumumul(10,10)"),
            vec![Mul(300, 5), Mul(10, 10)]
        );
        assert_eq!(parse("mul(300,5)mulumul(10,10"), vec![Mul(300, 5)]);
        assert_eq!(
            parse("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)]
        );

        assert_eq!(parse("mul ( 2 , 4 )"), vec![]);
        assert_eq!(parse("mul%(#2X,4)"), vec![]);
    }
}
