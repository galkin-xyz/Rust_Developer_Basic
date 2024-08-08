#![allow(dead_code)]
/*
    Написать функцию, которая превращает число в строку по следующим правилам:
    1. Если число кратно 3, то возвращаем строку "Fizz"
    2. Если число кратно 5, то возвращаем строку "Buzz"
    3. Если число кратно и 3, и 5, то возвращаем строку "FizzBuzz"
    4. В остальных случаях возвращаем строку, содержащую данное число

    Написать функцию fizzbuzz_list, которая получает число `n: u32` и возвращает
    список строк, содержащих строковые представления fizzbuzz
    для чисел в диапазоне от 1 до n. Написать тесты.
*/

fn fizzbuzz(num: u32) -> String {
    let mut res = String::new();

    if num % 3 == 0 {
        res.push_str("Fizz");
    }
    if num % 5 == 0 {
        res.push_str("Buzz");
    }
    if res.is_empty() {
        res = num.to_string();
    }
    res
}

fn fizzbuzz_list(n: u32) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for i in 1..=n {
        res.push(fizzbuzz(i));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_works() {
        assert_eq!(&fizzbuzz(1), "1");
        assert_eq!(&fizzbuzz(3), "Fizz");
        assert_eq!(&fizzbuzz(5), "Buzz");
        assert_eq!(&fizzbuzz(7), "7");
        assert_eq!(&fizzbuzz(9), "Fizz");
        assert_eq!(&fizzbuzz(15), "FizzBuzz");
        assert_eq!(&fizzbuzz(30), "FizzBuzz");
        assert_eq!(&fizzbuzz(49), "49");
    }

    #[test]
    fn fizzbuzz_list_works() {
        let res = fizzbuzz_list(0);
        assert!(res.is_empty());

        let res = fizzbuzz_list(1);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "1");

        let res = fizzbuzz_list(15);
        assert_eq!(res.len(), 15);
        assert_eq!(res[0], "1");
        assert_eq!(res[1], "2");
        assert_eq!(res[2], "Fizz");
        assert_eq!(res[3], "4");
        assert_eq!(res[4], "Buzz");
        assert_eq!(res[5], "Fizz");
        assert_eq!(res[6], "7");
        assert_eq!(res[7], "8");
        assert_eq!(res[8], "Fizz");
        assert_eq!(res[9], "Buzz");
        assert_eq!(res[10], "11");
        assert_eq!(res[11], "Fizz");
        assert_eq!(res[12], "13");
        assert_eq!(res[13], "14");
        assert_eq!(res[14], "FizzBuzz");
    }
}
