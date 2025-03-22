
mod constructor;

use testing_basic;

#[test]
#[should_panic]
fn test0() {
    println!("!!!!~~~~~");
    testing_basic::ee0();
    constructor::test_panic_lib();
}
