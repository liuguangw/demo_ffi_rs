#[repr(C)]
#[derive(Debug)]
pub struct Foo;

///创建Foo对象
#[no_mangle]
pub extern "C" fn foo_new1() -> Box<Foo> {
    println!("> call foo_new1:");
    Box::new(Foo)
}

///创建Foo对象
#[no_mangle]
pub extern "C" fn foo_new2() -> *mut Foo {
    println!("> call foo_new2:");
    let f = Box::new(Foo);
    Box::into_raw(f)
}

///释放Foo对象
#[no_mangle]
pub extern "C" fn foo_delete(f: Option<Box<Foo>>) {
    println!("> call foo_delete:");
    dbg!(f);
}
