mod files;
mod foo;
pub use files::{free_file, free_string, open_file, read_file_as_string};
pub use foo::{foo_delete, foo_new1, foo_new2, Foo};

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
