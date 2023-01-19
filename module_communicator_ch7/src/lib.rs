pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod network {
    fn connect() {}
}

//There could be multiple modules in one file
mod client {
    fn connect() {}
    mod specialized_client {
        fn aaa() {}
    }
}

mod module_in_other_file;

mod declaration_of_module_name_module_will_not_be_in_file;

mod module_with_submodule;
