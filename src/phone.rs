use fastrand;

pub fn generate_phone() -> String {
    let prefix = "+1";
    let number = fastrand::i64(100_000_0000..1_000_000_0000).to_string();
    format!("{}{}", prefix, number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_phone() {
        let phone = generate_phone();
        println!("{phone}");
        assert!(!phone.is_empty());
        assert!(phone.starts_with("+1"));
        assert_eq!(phone.len(), 12);
    }
}
