#![allow(dead_code)]
/*
    Дан массив, который содержит n неповторяющихся чисел в диапазоне
    от 0 до n включительно.

    Написать функцию, которая вернёт единственное число, отсутствующее
    в данном массиве.

    Гарантируется, что числа в массиве не повторяются и все принадлежат
    заданному диапазону.
*/

fn missing_num(nums: &[i32]) -> i32 {
    let size: usize = nums.len();
    let mut state: Vec<bool> = Vec::new();

    for _ in 0..=size {
        state.push(true);
    }

    for i in 0..size {
        state[nums[i] as usize] = false;
    }

    let mut i: usize = 0;
    while i < state.len() {
        if state[i] {
            break;
        }
        i += 1;
    }
    i as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(missing_num(&[1, 2]), 0);
        assert_eq!(missing_num(&[1, 0, 4, 2]), 3);
        assert_eq!(missing_num(&[0, 4, 2, 5, 3, 6]), 1);
    }
}
