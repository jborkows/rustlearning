pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod box_usage;
mod ref_cell_as_mocks;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
