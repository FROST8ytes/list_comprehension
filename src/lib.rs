#[cfg(test)]
mod tests {
    use comp_macro::comp;

    #[test]
    fn simple_test() {
        let result = comp![x for x in [1, 2, 3]].collect::<Vec<_>>();
        assert_eq!(result, [1, 2, 3]);
    }

    #[test]
    fn mapping_test() {
        let result = comp![x + 1 for x in [1, 2, 3]].collect::<Vec<_>>();
        assert_eq!(result, [2, 3, 4])
    }

    #[test]
    fn condition_test() {
        let result = comp![x + 1 for x in [1, 2, 3] if x > 1].collect::<Vec<_>>();
        assert_eq!(result, [3, 4]);
    }
}
