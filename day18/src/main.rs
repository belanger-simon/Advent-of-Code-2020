use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

#[derive(Debug)]
enum Token {
    Number(u64),
    Add,
    Mul,
    Open,
    Close
}

#[derive(Debug, Clone)]
enum Expr {
    Number(u64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Group(Box<Expr>)
}

impl Expr {
    fn from_tokens(tokens: &Vec<Token>) -> Box<Expr> {
        let mut tokens = tokens.iter().collect::<VecDeque<_>>();
    
        fn parse_group(tokens: &mut VecDeque<&Token>) -> Box<Expr> {
            let mut exprs = Vec::new();

            while let Some(token) = tokens.pop_front() {
                match token {
                    Token::Number(n) => exprs.push(Box::new(Expr::Number(*n))),
                    Token::Add => {
                        let left = exprs.pop().unwrap();
                        let right = match tokens.pop_front() {
                            Some(Token::Number(n)) => Box::new(Expr::Number(*n)),
                            Some(Token::Open) => parse_group(tokens),
                            _ => panic!("Invalid tokens")
                        };

                        exprs.push(Box::new(Expr::Add(left, right)))
                    },
                    Token::Mul => {
                        let left = exprs.pop().unwrap();
                        let right = match tokens.pop_front() {
                            Some(Token::Number(n)) => Box::new(Expr::Number(*n)),
                            Some(Token::Open) => parse_group(tokens),
                            _ => panic!("Invalid tokens")
                        };

                        exprs.push(Box::new(Expr::Mul(left, right)))
                    },
                    Token::Open => exprs.push(parse_group(tokens)),
                    Token::Close => break
                }
            }

            Box::new(Expr::Group(exprs.pop().unwrap()))
        }
    
        parse_group(&mut tokens)
    }
}

fn main() {
    let input : Vec<Vec<_>> = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .take_while(|l| l != "")
        .map(|l| l.chars().filter_map(|c| {
            match c {
                '+' => Some(Token::Add),
                '*' => Some(Token::Mul),
                '(' => Some(Token::Open),
                ')' => Some(Token::Close),
                c @ '0'..='9' => Some(Token::Number(c.to_digit(10).unwrap() as u64)),
                _ => None
            }
        }).collect())
        .collect();
    
    let answer_one = input.iter()
        .map(|tokens| {
            let expr = Expr::from_tokens(tokens);

            fn evaluate_expr(expr: Box<Expr>) -> u64 {
                match *expr {
                    Expr::Group(expr) => evaluate_expr(expr),
                    Expr::Add(l, r) => evaluate_expr(l) + evaluate_expr(r),
                    Expr::Mul(l, r) => evaluate_expr(l) * evaluate_expr(r),
                    Expr::Number(n) => n
                }
            }

            evaluate_expr(expr)
        })
        .sum::<u64>();

    println!("Part One: {}", answer_one);

    let answer_two = input.iter()
        .map(|tokens| {
            let expr = Expr::from_tokens(tokens);

            // Rotate a Add node that has a mul subtree with a number so that all Add node are deeper than Mul node
            fn prioritize_add(expr: &Box<Expr>) -> Box<Expr> {
                match &**expr {
                    Expr::Group(expr) =>  Box::new(Expr::Group(prioritize_add(&expr))),
                    Expr::Number(n) => Box::new(Expr::Number(*n)),
                    Expr::Mul(l, r) => {
                        Box::new(Expr::Mul(prioritize_add(&l), prioritize_add(&r)))
                    },
                    Expr::Add(l, r) => {
                        let l = prioritize_add(&l);
                        let r = prioritize_add(&r);

                        match &*l {
                            Expr::Mul(ml, mr) => {
                                Box::new(
                                    Expr::Mul(
                                        prioritize_add(&ml),
                                        Box::new(Expr::Add(r, prioritize_add(&mr)))
                                    )
                                )
                            },
                            _ => {
                                Box::new(Expr::Add(prioritize_add(&l), prioritize_add(&r)))
                             }
                        }
                    }
                }
            }

            let expr = prioritize_add(&expr);

            fn evaluate_expr(expr: Box<Expr>) -> u64 {
                match *expr {
                    Expr::Group(expr) => evaluate_expr(expr),
                    Expr::Add(l, r) => evaluate_expr(l) + evaluate_expr(r),
                    Expr::Mul(l, r) => evaluate_expr(l) * evaluate_expr(r),
                    Expr::Number(n) => n
                }
            }

            evaluate_expr(expr)
        })
        .sum::<u64>();

    println!("Part Two: {}", answer_two);
}