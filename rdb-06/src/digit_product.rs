#![allow(dead_code)]
/*
    Написать функцию, которая будет вычислять произведение цифр числа,
    при этом цифра 0 игнорируется. Затем повторить операцию с результатом
    произведения, пока не получится число, состоящее из одной цифры.
*/

fn digit_product(n: u32) -> u8 {
    let mut cur_n: u32 = n;

    while cur_n >= 10 {
        let mut res: u32 = 1;
        while cur_n > 0 {
            let ost: u32 = cur_n % 10;
            if ost > 0 {
                res *= ost;
            }
            cur_n /= 10;
        }
        cur_n = res;
    }
    cur_n as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2); // 9*8*7=504, 5*4=20, 2
        assert_eq!(digit_product(123456), 4); // 1*2*3*4*5*6=720, 7*2=14, 1*4=4
        assert_eq!(digit_product(123454321), 6); // 1*2*3*4*5*4*3*2*1=2880, 2*8*8=128, 1*2*8=16, 1*6=6
    }
}
