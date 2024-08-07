pub type SignedCounter = isize;
pub type UnsignedCounter = usize;

pub fn default_signed_counter() -> SignedCounter {
    0
}

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}
pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

#[cfg(test)]
mod tests {
    use crate::counter::*;

    #[test]
    fn default_values() {
        assert_eq!(default_signed_counter(), 0);
        assert_eq!(default_unsigned_counter(), 0);
    }

    #[test]
    fn next_values() {
        assert_eq!(next_signed(-10), -9);
        assert_eq!(next_signed(102030), 102031);
        assert_eq!(next_unsigned(5), 6);
        assert_eq!(next_unsigned(9999), 10000);
    }

    #[test]
    fn prev_values() {
        assert_eq!(prev_signed(100), 99);
        assert_eq!(prev_signed(-567), -568);
    }
}
