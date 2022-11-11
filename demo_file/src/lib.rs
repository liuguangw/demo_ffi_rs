mod files;
pub use files::{free_file, free_string, open_file, read_file_as_string};

///简单的加法
#[no_mangle]
pub extern "C" fn add(left: u32, right: u32) -> u32 {
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
