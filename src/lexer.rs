use crate::number::Number::{Float, Int};
use crate::operator::Equals::EqualsSign;
use crate::operator::Operator::{And, Divide, Equals, GreaterThan, LessThan, Minus, Multiply, No, NotEquals, Or, Plus};

use crate::tokens::Token;
use crate::tokens::Token::{LeftCurlyBrace, RightCurlyBrace, StringLiteral};

pub fn lex(input: &str) -> Vec<Token>
{
    let mut tokens = Vec::new();


    let mut i = 0;
    while i < input.len() {

        if let Some(mut character) = input.chars().nth(i){


            match character {


                // ignores white space
                ' ' => {}
                gotten @( '"' | '\'') =>{
                    let mut text = String::new();
                    while let Some(next_char)
                        = input.chars().nth(i + 1)
                    {

                        i += 1;
                        if next_char != gotten{
                            text.push(next_char);
                        } else{
                            break;
                        }
                    }
                    tokens.push(StringLiteral(text));
                },
                '{' =>{
                    tokens.push(LeftCurlyBrace);
                },
                '}' => {
                    tokens.push(RightCurlyBrace);
                }
                // converting numbers to tokens
                '0'..='9' => {
                    let mut my_str = String::from(character);

                    let mut is_float = false;
                    let mut look_for_characters = |i: &mut usize|{
                        while let Some(next_char)
                            = input.chars().nth(*i + 1)
                            && (('0'..='9').contains(&next_char)) {
                            character = next_char;
                            my_str.push(character);
                            *i += 1;
                        }
                    };
                    look_for_characters(&mut i);
                    if let Some('.')  = input.chars().nth(i + 1) {
                        i += 1;
                        is_float = true;
                    }
                    look_for_characters(&mut i);
                    if !is_float{
                        let as_num : i32 = my_str.parse().expect(format!("Unable to parse {my_str} as number").as_str());
                        tokens.push(Token::Number(Int(as_num)));
                    } else {
                        let as_num : f32 = my_str.parse().expect(format!("Unable to parse {my_str} as number").as_str());
                        tokens.push(Token::Number(Float(as_num)));
                    }

                },
                ';' => {
                    tokens.push(Token::SemiColon);
                }
                '<' => {
                    tokens.push(Token::Operator(LessThan));
                }
                '>' => tokens.push(Token::Operator(GreaterThan)),
                '!' => {
                    if let Some('=') = input.chars().nth(i + 1){
                        tokens.push(Token::Operator(NotEquals));
                        i += 1;
                    }
                }
                '=' => {
                    if let Some('=') = input.chars().nth(i + 1){
                        tokens.push(Token::Operator(Equals));
                        i += 1;
                    } else{
                        tokens.push(Token::Assign);
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut my_str = String::from(character);
                    let  possible_other_characters =('a'..='z').chain('A'..='Z').chain('0'..'9').chain(['-','_']).collect::<Vec<_>>();
                    while let Some(next_char)
                        = input.chars().nth(i + 1)
                        && possible_other_characters.contains(&next_char) {
                        character = next_char;
                        my_str.push(character);
                        i += 1;

                    }
                    match my_str.as_str() {
                        "bag" => {
                            tokens.push(Token::Bag);
                        },
                        "yap" => tokens.push(Token::Yap),
                        "if" => tokens.push(Token::If),
                        "else" => tokens.push(Token::Else),
                        "is" => tokens.push(Token::Operator(Equals)),
                        "no" => tokens.push(Token::Operator(No)),
                        "cap" => tokens.push(Token::Cap),
                        "fax" => tokens.push(Token::Fax),
                        "while" => tokens.push(Token::While),
                        "and" => tokens.push(Token::Operator(And)),
                        "or" => tokens.push(Token::Operator(Or)),
                        other => tokens.push(Token::Name(my_str))
                    }


                }
                '+' => {
                    tokens.push(Token::Operator(Plus));
                },
                '-' => {
                    tokens.push(Token::Operator(Minus));
                },
                '\n' | '\r' => {

                }
                '*' => {
                    tokens.push(Token::Operator(Multiply));
                },
                '/' => {
                    tokens.push(Token::Operator(Divide));
                },
                '(' => {
                    tokens.push(Token::LeftBracket);
                },
                ')' => {
                    tokens.push(Token::RightBracket);
                },
                other => {
                    panic!("Unrecognized character: {}", character);
                }

            }
            i += 1
        }
    }

    return tokens;
}
