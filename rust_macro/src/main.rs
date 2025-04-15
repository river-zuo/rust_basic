macro_rules! add {
    ($a: expr) => {
        $a
    };
    ($a: expr, $b: expr) => {
        $a + $b
    };
    ($a: expr, $($b: tt)*) => {
        $a + add!($($b)*)
    };
}

#[derive(Default)]
pub struct CommandBuilder {
    executable: String,
    args: Vec<String>,
    current_dir: String,
}

impl CommandBuilder {
    
}

#[derive(Debug)]
pub struct Aaa();

fn main() {

    // add!(1, 2, 3, 4);
    
    // println!("add!_ {}", add!(1, 2));
    println!("add!_ {}", add!(1));
    println!("add!_ {}", add!(1, 2, 4, 5, 6, 7, 8));
    
}

