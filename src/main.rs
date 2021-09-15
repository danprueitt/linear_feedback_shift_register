fn main() {
    let mut state: u128 = (1 << 127) | 1;
    loop {
        print!("{}", state & 1);
        let new_bit = (state ^ (state >> 1) ^ (state >> 2) ^ (state >> 7)) & 1;
        state = (state >> 1) | (new_bit << 127);
    }
}
