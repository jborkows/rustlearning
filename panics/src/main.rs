use std::panic;
fn main() {
    catch_unwind_usage();
    setting_panic_hooks();
}

fn setting_panic_hooks() {
    let _panic_hook = panic::take_hook();
    panic::set_hook(Box::new(|panic_struct: &panic::PanicInfo| {
        println!("Now this will be printed! {:?}", panic_struct.location())
    }));
    panic!("Haha I will be not printed");
    // panic::set_hook(_panic_hook);
}

fn catch_unwind_usage() {
    let result = panic::catch_unwind(|| {
        println!("Hello");
    });
    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("AAAA");
    });
    assert!(result.is_err());
}
