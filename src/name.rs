use fastrand;

const FIRST_NAMES: &[&str] = &["James", "Mary", "John", "Patricia"];
const LAST_NAMES: &[&str] = &["Smith", "Johnson", "Williams", "Brown"];

pub fn generate_name() -> String {
    let first_name = FIRST_NAMES[fastrand::usize(0..FIRST_NAMES.len())];
    let last_name = LAST_NAMES[fastrand::usize(0..LAST_NAMES.len())];
    format!("{} {}", first_name, last_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_name() {
        let name = generate_name();
        println!("{name}");
        assert!(!name.is_empty());
        assert!(name.contains(" "));
    }
}
