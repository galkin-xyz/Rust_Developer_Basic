#![allow(dead_code)]
/*
    Последовательностью Фибоначчи называется последовательность чисел,
    которая удовлетворяет следующим условиям:
    - элемент последовательности с индексом 0 - число 0
    - элемент с индексом 1 - число 1
    - каждый последующий элемент равен сумме двух предыдущих.

    0, 1, 1, 2, 3, 5, 8, 13, 21 ...

    Написать функцию, которая вычислит элемент последовательности с индексом n.

    * Написать вторую функцию, которая вернёт последовательность Фибонначи
      от первого элемента до n-ого. Написать тесты
*/

fn fib(n: u32) -> u32 {
    let mut n_2: u32 = 0;
    let mut n_1: u32 = 1;
    let mut next: u32 = 0;

    match n {
        0 => n_2,
        1 => n_1,
        _ => {
            for _ in 2..=n {
                next = n_2 + n_1;
                n_2 = n_1;
                n_1 = next;
            }
            next
        }
    }
}

fn fib_list(n: u32) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::<u32>::new();

    for i in 0..=n {
        res.push(fib(i))
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_works() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(8), 21);
    }

    #[test]
    fn fib_list_works() {
        let res = fib_list(7);

        assert_eq!(res.len(), 8);
        assert_eq!(res[0], 0);
        assert_eq!(res[1], 1);
        assert_eq!(res[2], 1);
        assert_eq!(res[3], 2);
        assert_eq!(res[4], 3);
        assert_eq!(res[5], 5);
        assert_eq!(res[6], 8);
        assert_eq!(res[7], 13);
    }
}
