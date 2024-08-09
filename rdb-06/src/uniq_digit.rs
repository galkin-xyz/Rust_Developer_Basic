#![allow(dead_code)]
/*
    Дана строка, состоящая только из цифровых символов. В данной строке
    есть одна цифра, которая не повторяется. Написать функцию,
    которая найдёт эту цифру и вернёт её.

    * Написать похожую функцию, но только на этот раз в данной строке
    могут присутствовать любые символы, а уникальная цифра может отсутствовать.
    Но если присутсвует, то не больше одной. Написать тесты.
*/

const RADIX: u32 = 10;
fn uniq_digit(s: &str) -> u8 {
    let mut state: [u8; 10] = [0; 10];

    for ch in s.chars() {
        let digit = ch.to_digit(RADIX).unwrap() as usize;
        if state[digit] <= 1 {
            state[digit] += 1;
        }
    }
    let mut digit: usize = 0;
    while digit < 10 {
        if state[digit] == 1 {
            break;
        }
        digit += 1
    }
    digit as u8
}

fn uniq_digit_any(s: &str) -> Option<u8> {
    let mut state: [u8; 10] = [0; 10];

    for ch in s.chars() {
        if let Some(digit) = ch.to_digit(RADIX) {
            if state[digit as usize] <= 1 {
                state[digit as usize] += 1;
            }
        }
    }
    for (digit, item) in state.iter().enumerate() {
        if *item == 1 {
            return Some(digit as u8);
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn uniq_digit_works() {
        assert_eq!(uniq_digit("3"), 3);
        assert_eq!(uniq_digit("010"), 1);
        assert_eq!(uniq_digit("47343077"), 0);
        assert_eq!(uniq_digit("123454321"), 5);
        assert_eq!(uniq_digit("0987654321234567890"), 1);
        assert_eq!(uniq_digit("4444444444424444444444444"), 2);
    }

    #[test]
    fn uniq_digit_any_works() {
        assert_eq!(uniq_digit_any("3"), Some(3));
        assert_eq!(uniq_digit_any("ab123c4d54h321"), Some(5));
        assert!(uniq_digit_any("112233").is_none());
        assert!(uniq_digit_any("abcde").is_none());
    }
}
