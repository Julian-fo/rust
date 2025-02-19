#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(5, 5);
    }
}

pub mod adder {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}
