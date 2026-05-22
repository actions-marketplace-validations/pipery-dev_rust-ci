fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn intentional_failure() {
        assert_eq!(1, 2);
    }
}
