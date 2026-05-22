pub fn greeting(name: &str) -> String {
    let message = format!("hello, {name}");
    if cfg!(feature = "shout") {
        message.to_uppercase()
    } else {
        message
    }
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn applies_shout_feature() {
        assert_eq!(greeting("pipery"), "HELLO, PIPERY");
    }
}
