pub type Pair = (i32, i32);

pub fn default_pair() -> Pair {
    (0, 0)
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_val() {
        assert_eq!(default_pair(), (0, 0));
    }

    #[test]
    fn vector_sum() {
        assert_eq!(pair_vector_sum((-5, 25), (-10, 100)), (-15, 125));
    }

    #[test]
    fn scalar_sum() {
        assert_eq!(pair_scalar_sum((0, -10), (30, 50)), 70);
    }
}
