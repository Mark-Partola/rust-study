#[cfg(test)]
mod test {
    use crate::modules_1::arrays::chunk;

    #[test]
    fn test_chunk() {
        let data = (1..10).collect::<Vec<_>>();
        let result = chunk(&data, 2);
        let expected = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9]];

        assert_eq!(result.len(), 5);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_chunk_empty() {
        let data = vec![];
        let result = chunk(&data, 2);
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(result.len(), 0);
        assert_eq!(result, expected);
    }
}
