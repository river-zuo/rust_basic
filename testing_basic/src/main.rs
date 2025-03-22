use testing_basic::ee0;



fn main() {
    println!("Hello, world!");

    use testing_basic;
    use testing_basic::testing;
    testing_basic::ee0();
    testing::cc();
    
    assert_eq!(1, 1);
    assert_ne!(1, 2);
    assert!(true);
    debug_assert!(true);
    
}

fn test_add(i: i32) -> i32 {
    i + 1
}

pub fn test_panic(num: i32) {
    if num >= 0 {
        panic!("true message");
    } else {
        panic!("false message")
    }
}

#[cfg(test)]
mod tests {
    use criterion::Bencher;

    use crate::{test_add, test_panic};


    #[test]
    fn test0() {
        assert_eq!(test_add(1), 2);

        let left = 1 + 2;
        let right = 3;
        assert_eq!(left, right, "we expect equal but got {} and {}", left, right);
    }

    #[test]
    #[should_panic]
    fn test1() {
        // println!("should panic");
        panic!("panic~~!!!!!!");
    }

    #[test]
    #[should_panic(expected = "true message")]
    fn test2() {
        test_panic(1);
    }

    #[test]
    fn test_case4() {
        println!("run me only.");
    }

    // #[bench]
    // fn bench_add_two(b: &mut Bencher) {
    //     {
    //         b.iter(||
    //             for i in 0..10000 {
    //                 println!("")
    //             }
    //         );
    //     }
    // }

}

