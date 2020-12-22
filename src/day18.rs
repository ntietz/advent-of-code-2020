use std::fs;

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let input = fs::read_to_string("inputs/day18.txt").unwrap();
    let solution: u64 = input.lines().map(evaluate).sum();
    println!("day18.part1.solution = {}", solution);
}

pub fn part2() {
    let input = fs::read_to_string("inputs/day18.txt").unwrap();
    let solution: u64 = input.lines().map(evaluate_precedence).sum();
    println!("day18.part2.solution = {}", solution);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operand {
    Plus,
    Times,
}

type Number = u64;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Number(Number),
    Operand(Operand),
    LeftParen,
    RightParen,
}

fn evaluate(expr: &str) -> u64 {
    let mut tokens = tokenize(expr);
    evaluate_tokens(&mut tokens, false)
}

fn evaluate_precedence(expr: &str) -> u64 {
    let mut tokens = tokenize(expr);
    evaluate_tokens(&mut tokens, true)
}

fn evaluate_tokens(tokens: &mut Vec<Token>, precedence: bool) -> u64 {
    while tokens.iter().filter(|&c| *c == Token::LeftParen).count() > 0 {
        if let Some(start_idx) = tokens.iter().position(|t| *t == Token::LeftParen) {
            let mut depth = 1;
            let mut stop_idx = start_idx + 1;
            while depth > 0 {
                match tokens[stop_idx] {
                    Token::LeftParen => {
                        depth += 1;
                    }
                    Token::RightParen => {
                        depth -= 1;
                    }
                    _ => {}
                }
                stop_idx += 1;
            }
            let sub = evaluate_tokens(
                &mut tokens[start_idx + 1..stop_idx - 1].to_vec(),
                precedence,
            );
            tokens[start_idx] = Token::Number(sub);
            tokens.drain(start_idx + 1..stop_idx);
        }
    }

    while tokens.len() > 1 {
        let next_plus = tokens
            .iter()
            .position(|&t| t == Token::Operand(Operand::Plus));
        let next_times = tokens
            .iter()
            .position(|&t| t == Token::Operand(Operand::Times));
        let next_idx = match (precedence, next_plus, next_times) {
            (false, _, _) => 0,
            (true, Some(idx), _) => idx - 1,
            (true, None, Some(idx)) => idx - 1,
            _ => panic!("What operator hath snuck in?"),
        };

        tokens[next_idx] = match (tokens[next_idx], tokens[next_idx + 1], tokens[next_idx + 2]) {
            (Token::Number(x), Token::Operand(Operand::Plus), Token::Number(y)) => {
                Token::Number(x + y)
            }
            (Token::Number(x), Token::Operand(Operand::Times), Token::Number(y)) => {
                Token::Number(x * y)
            }
            _ => panic!("Huh, something else snuck in!"),
        };
        tokens.drain(next_idx + 1..next_idx + 3);
    }

    match tokens[0] {
        Token::Number(x) => x,
        _ => panic!("We should have just a number left :("),
    }
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut acc = 0;

    for c in expr.chars() {
        match c {
            '+' => tokens.push(Token::Operand(Operand::Plus)),
            '*' => tokens.push(Token::Operand(Operand::Times)),
            '(' => tokens.push(Token::LeftParen),
            ')' => {
                if acc != 0 {
                    tokens.push(Token::Number(acc))
                }
                acc = 0;
                tokens.push(Token::RightParen)
            }
            ' ' => {
                if acc != 0 {
                    tokens.push(Token::Number(acc))
                }
                acc = 0;
            }
            c if c.is_ascii_digit() => {
                acc = (acc * 10) + c.to_digit(10).unwrap() as u64;
            }
            _ => {}
        }
    }

    if acc != 0 {
        tokens.push(Token::Number(acc));
    }

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_simple_expressions() {
        assert_eq!(evaluate("1 + 2 * 3"), 9);
    }

    #[test]
    fn handles_parentheses() {
        assert_eq!(evaluate("1 + (2 * 3) + 4"), 11);
    }

    #[test]
    fn handles_starting_parens() {
        assert_eq!(
            evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn handles_double_ending_parens() {
        assert_eq!(evaluate("8 * 3 * (6 + (6 * 8 * 2)) + 9 + 9 * 3"), 7398);
    }

    #[test]
    fn parses_right_number_of_tokens() {
        assert_eq!(dbg!(tokenize("1 + (2 * (3 * 4))")).len(), 11);
    }

    #[test]
    fn parses_via_tokens() {
        assert_eq!(
            evaluate_tokens(
                &mut tokenize("8 * 3 * (6 + (6 * 8 * 2)) + 9 + 9 * 3"),
                false
            ),
            7398
        );
    }
}
