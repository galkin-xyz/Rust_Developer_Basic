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
