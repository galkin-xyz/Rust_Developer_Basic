#![allow(dead_code)]
/*
    Дана строка, состоящая только из символов '{', '}', '(', ')', '[', ']'.
    Такая строка является корректной, если:
    - каждой открывающей скобке соответствует закрывающая того же типа
    - соблюдается порядок закрытия скобок
    - для каждой закрывающей скобки есть соответствующая открывающая пара

    Написать функцию, которая проверит корректность данной строки.
*/
use std::{char, collections::HashMap};

fn validate_paren(s: &str) -> bool {
    let open: &str = "({[";
    let pair: HashMap<char, char> = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);
    let mut state: Vec<char> = Vec::new();

    for ch in s.chars() {
        if open.contains(ch) {
            state.push(ch);
        } else {
            let expected: char = *pair.get(&ch).unwrap();
            if let Some(val) = state.pop() {
                if val != expected {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    state.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(validate_paren("()"), true);
        assert_eq!(validate_paren("()[]{}"), true);
        assert_eq!(validate_paren("({[]()})"), true);
        assert_eq!(validate_paren("(}"), false);
        assert_eq!(validate_paren("()]"), false);
        assert_eq!(validate_paren("(){"), false);
    }
}
