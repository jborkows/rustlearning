pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod time_live_annotations;
pub mod using_maps;
pub mod using_strings;
pub mod using_vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
