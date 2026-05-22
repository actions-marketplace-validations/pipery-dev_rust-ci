pub fn answer() -> i32 {
    41
}

#[cfg(test)]
mod tests {
    use super::answer;

    #[test]
    fn catches_wrong_answer() {
        assert_eq!(answer(), 42);
    }
}
