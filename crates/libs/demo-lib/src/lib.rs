#[must_use]
pub fn rand_string(len: usize) -> String {
    const CHAR_SET: &[u8] = "abcdefghijklmnopqrstuvwxyz0123456789".as_bytes();
    const CHAR_SET_LEN: usize = CHAR_SET.len();
    (0..len)
        .map(|_| CHAR_SET[rand::random::<usize>() % CHAR_SET_LEN] as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rand_string() {
        let s = rand_string(0);
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
        assert_eq!(s, "");
    }
}
