use super::pool;

pub fn entropy(s: &str) -> f64 {
    let pool_size = pool::size(s);
    let length = s.len();
    let entropy = length as f64 * f64::from(pool_size).log2();
    println!(
     "length: {}, pool size {}, entropy {}",
     length, pool_size, entropy
    );
    entropy
}

#[cfg(test)]
mod tests {
    use super::entropy;

    #[test]
    fn test_entropy() {
        assert_eq!(entropy("password") as u64, 37);
        assert_eq!(entropy("Password") as u64, 45);
        assert_eq!(entropy("Password$") as u64, 57);
        assert_eq!(entropy("Password123") as u64, 65);
        assert_eq!(entropy("Password123$") as u64, 78);
    }
}
