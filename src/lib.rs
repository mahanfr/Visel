#[no_mangle]
pub extern "C" fn dogpoop(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = dogpoop(2, 2);
        assert_eq!(result, 4);
    }
}
