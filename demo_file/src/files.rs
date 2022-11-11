use std::{
    ffi::{c_char, c_int, c_void, CStr, CString},
    fs::File,
    io::Read,
};

///打开文件
///
/// 本函数会修改 `item_ptr` 参数的值,
/// 如果打开文件成功, `item_ptr` 将保存文件的指针
/// 如果打开失败, 则会保存错误信息的字符串的内存地址
/// 返回`0`表示成功,`-1`表示失败
///
/// # Safety
/// 执行次函数之后,注意要手动释放 `item_ptr` 指向的内存
#[no_mangle]
pub unsafe extern "C" fn open_file(filename: *const c_char, item_ptr: *mut *mut c_void) -> c_int {
    let slice = unsafe { CStr::from_ptr(filename) };
    match File::open(slice.to_str().unwrap()) {
        Ok(file) => {
            *item_ptr = Box::into_raw(Box::new(file)) as *mut c_void;
            0
        }
        Err(err) => {
            let message = format!("{:?}", err);
            let message = CString::new(message).unwrap();
            *item_ptr = message.into_raw() as *mut c_void;
            -1
        }
    }
}

///读取文件内容
///
/// 本函数会修改 `item_ptr` 参数的值,
/// 如果读取文件成功, `item_ptr` 将保存文件内容字符串的指针
/// 如果读取失败, 则会保存错误信息的字符串的内存地址
/// 返回`0`表示成功,`-1`表示失败
///
/// # Safety
/// 执行次函数之后,注意要手动释放 `item_ptr` 指向的内存
#[no_mangle]
pub unsafe extern "C" fn read_file_as_string(
    file_ptr: *mut File,
    item_ptr: *mut *mut c_char,
) -> c_int {
    let Some(file) = file_ptr.as_mut() else{
        let message = CString::new("file_ptr is null").unwrap();
        *item_ptr = message.into_raw();
        return -1;
    };
    let mut file_content = String::new();
    if let Err(err) = file.read_to_string(&mut file_content) {
        let message = CString::new(format!("{err:?}")).unwrap();
        *item_ptr = message.into_raw();
        return -1;
    };
    let file_content = CString::new(file_content).unwrap();
    *item_ptr = file_content.into_raw();
    0
}

///释放文件资源
///
/// # Safety
/// 释放由rust分配的内存
#[no_mangle]
pub extern "C" fn free_file(file: Option<Box<File>>) {
    println!("> call free_file:");
    dbg!(file);
}

///释放字符串资源
///
/// # Safety
/// 释放由rust分配的内存
#[no_mangle]
pub unsafe extern "C" fn free_string(string_ptr: *mut c_char) {
    println!("> call free_string:");
    let c_string_value = unsafe { CString::from_raw(string_ptr) };
    dbg!(c_string_value.to_str().unwrap());
}
