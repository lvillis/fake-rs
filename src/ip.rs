use fastrand;

pub fn generate_ip() -> String {
    format!(
        "{}.{}.{}.{}",
        fastrand::u8(..),
        fastrand::u8(..),
        fastrand::u8(..),
        fastrand::u8(..)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ip() {
        let ip = generate_ip();
        println!("{ip}");
        assert!(!ip.is_empty());
        assert_eq!(ip.matches('.').count(), 3);
    }
}
