pub fn sum(vec: Vec<i32>) -> i32 {
    return vec.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(sum(vec![2, 5, 1, 8, 9, 13]), 38)
    }
}