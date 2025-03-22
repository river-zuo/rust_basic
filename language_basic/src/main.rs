use std::{collections::HashMap, error::Error, f32::consts::E, fs::File, io::{self, Read, Write}, net::{AddrParseError, IpAddr, Ipv4Addr}, ops::{Add, Deref}};

use num::{traits::float, Complex};

fn _greet() {
    let cn = "你好, 世界";
    let en = "hello, world";
    let lans = [cn, en];
    for lan in lans {
        println!("{}", lan);
    }
}

macro_rules! _say_hello {
    () => {
        println!("macro rule Hello!");
    };
}

fn _test_fn() -> String {
    String::from("hello world")
}

const W: i32 = 11;

fn _data_struct() -> () {
     // say_hello!();
    // greet();
    // println!("{}", test_fn());

    let mut x = 4;
    x = 3;

    const W: u32 = 11;
    const Y: i32 = 33;

    let (x, y) = (2, 3);
    let s1 = String::from("cc");
    let s2 = s1;
    println!("{}", s2);

    let c = 1;
    let c1 = c;
    println!("{}", c);

    let one_million: i64 = 1_000_000;

    let forty_twos = [42.0, 42f32, 42.0f32];
    println!("{:.2}", forty_twos[0]);

    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("undefined");
    }
    
    let a = Complex {re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let re = a + b;
    println!("{} + {}i", re.re, re.im);

    let c = 'z';
    let c1 = 'Z';
    let c2 = '国';
    let c3 = '🤭';
    println!("size_of_c {}", size_of_val(&c));
    println!("size_of_c1 {}", size_of_val(&c1));
    println!("size_of_c2 {}", size_of_val(&c2));
    println!("size_of_c3 {}", size_of_val(&c3));

    let s = String::from("hello");
    let len = s.len();
    let s1 = &s[0..s.len()];
    let s2 = &s[..];
    println!("{}", s1);
    println!("{}", s2);

    println!("=============================================");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let x1 = x.0;
    let x2 = x.1;
    let x3 = x.2;
    println!("{:?}", x);
    println!("{}", x.0);

    #[derive(Debug)]
    struct Stu {
        weight: u32,
        age: u32,
    }

    let l = Stu {
        weight: 64,
        age: 18,
    };
    println!("l _> {:?}", l);

    // 元组结构体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", black);

    // 单元结构体, 只关心行为
    struct AlwaysE;
    trait T1 {}
    impl T1 for AlwaysE {}

    struct Ipv4Addr {

    }

    struct Ipv6Addr {

    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    enum Message {
        get(String, String),
        put(String),
    }

    let s_n = Some(5);
    let s_s = Some("ss");
    let a_n: Option<i32> = None;

    let one = [1, 2, 3];
    let two = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2 = [0; 3];

    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];
    
}

fn _control_loop() -> () {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    println!("{}, {}, {}", r1, r2, r3);

    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);

    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world", s2);

    let n = 6;
    if n % 4 == 0 {
        println!("_0")
    } else if n % 4 == 1 {
        println!("_1")
    } else if n % 4 == 2 {
        println!("_2")
    } else {
        println!("_else")
    }

    let mut coll = [1, 2, 3, 4, 5];
    for i in 0..coll.len() {
        let item = coll[i];
        println!("item: {}", item);
    }

    for item in &mut coll.iter_mut() {
        *item = *item + 1;
    }

    for i in 0..coll.len() {
        let item = coll[i];
        println!("item -> {}", item);
    }
    let mut n = 0;
    while n <= 5 {
        println!("num n:{}", n);
        n = n + 1;
    }

    let mut arr: Vec<u32> = Vec::new();
    arr.push(1);
    arr.push(2);
    arr.push(3);

    println!("{:?}", arr);
    // let a = arr.pop();
    while let Some(pop) = arr.pop() {
        println!("arr_ {}", pop);
    }

    println!("{:?}", arr);

    let mut counter = 0;
    let re = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(counter, 10);
    assert_eq!(re, 20)


}

fn _match() -> () {
    let x = Some(15);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("match y , = {:?}", y),
        _ => println!("match x is {:?}",x ),
    }


    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something elsa"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point{ x: 0, y: 7 };
    match p {
        Point{x, y: 0} => println!("One the x axis at {}", x),
        Point{x: 0, y} => println!("One the y axis at {}", y),
        Point{x, y} => println!("One neither axis:({}, {})", x, y),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (a1, _, a3, _, a5) => {
            println!("some numbers: a1_{}, a2_{}, a3_{}", a1, a3, a5)
        }
    }
    match numbers {
        (a1, .., a5) => {
            println!("start_ {}, end_ {}", a1, a5);
        }
    }
}

fn _lifecircle() -> () {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);
    // let s1 = "cronsdb";
    // let re;
    // let s2 = String::from("timeseries database");
    // re = longest(s1, s2.as_str());
    // println!("the long is {}", re);

    // let s1 = "cronsdb";
    // let re;
    // {
    //     let s2 = String::from("timeseries database");
    //     re = longest(s1, s2.as_str());
    // }
    // println!("the long is {}", re);

    let i;
    {
        let hi = "hello world";
        let word = hi.split(' ').next().unwrap();
        i = Test {
            temp: word,
        };
    }
    println!("{:?}", i);

    // let a;
    // {
    //     a = "~~~";
    // }
    // println!("{}", a);

    // let c;
    // {
    //     c = String::from("value");
    // }
    // println!("{}", c);

    // 静态生命周期
    // 在Rust中有一个非常特殊的生命周期, 那就是'static 拥有该生命周期的引用可以和整个程序活的一样久
    // 在之前我们学过字符串字面量， 提到过它是被硬编码进Rust的二进制文件中， 因此这些字符串变量全部具有'static的生命周期
    let l: &'static str = "hello world";
    
    let one = "ccc";
    let aa = one_param(one);

    let a = Foo::Qux(10);

    if let Foo::Bar = a {
        println!("match Foo::bar");
    } else if let Foo::Baz = a {
        println!("match Foo::baz");
    } else {
        println!("match others");
    }

    match a {
        Foo::Bar => println!("Foo::Bar"),
        Foo::Baz => println!("Foo::Baz"),
        Foo::Qux(a) => println!("match {:?}", a),
    }

}

fn _0307() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::new(10, 15);
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // rect1.take_ownership();
    println!("{}", rect1.width);
    println!("{}", rect1.width());
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
    #[derive(Debug)]
    enum Action {   // enum 像struct 去定义方法?
        Add,
        Subtract,
    }

    impl Action {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Action::Add => x + y,
                Action::Subtract => x - y,
            }
        }
    }
    println!("========================================================================");
    let ac = Action::Add;
    let ab = Action::Subtract;
    let mut re = ac.run(1, 2);
    println!("{:?} {}", ac, re);
    re = ab.run(1, 2);
    println!("{:?} {}", ab, re);
    Action::Add.run(1, 2);

    let t = add(1_u32, 2_u32);
    add(1_i8, 2_i8);

    let c = Point{ x: 10_i32, y: 20_u32 };
    let d = Point{ x: "10_i16", y: "ccc" };
    let c = c.mixup(d);
    let e = Point{ x: 3.1_f32, y: 11_f32 };
    let e1 = Point{ x: 3.1_f32, y: 11_f64 };
    println!("========================================================================");
    
    let mut en = EmptyNde { x: 1_f32, y: 11_f32 };
    en.draw();
    en.move_to(1.1, 2.2);
    en.draw();
    
    draw2(&en);
    draw1(Box::new(en));
    
    let mut cc = Counter{ num: 0 };
    // cc.next();
    while cc.num < 10 {
        println!("num: {}", cc.next().unwrap());
    }
    // println!("num_ {}", cc.count());

    let p1 = Point{ x: 1_i32, y: 15 };
    let p2 = Point{ x: 1_i32, y: 151 };
    let p3 = p1 + p2;
    assert_eq!(Point{ x: 1_i32, y: 15 } + Point{ x: 1_i32, y: 151 }, p3);

    let hh = Human;
    hh.fly();
    Pilot::fly(&hh);
    Wizad::fly(&hh);
    <Human as Pilot>::fly(&hh);
    Human::work();
    <Human as Pilot>::work();

    let mut values = vec![1, 2 ,3];
    for v in values.iter_mut() {
        println!("{}", v);
    }
    println!("{:?}", values);
    // let a = values.iter();
    // let iter = values.iter();
    let vv = vec![1, 2, 3, 4, 5];
    let a = vv.iter().sum::<i32>();
    println!("a_ {}", a);
    let v1: Vec<_> = vv.iter().map(|x| x + 1).collect();
    assert_eq!(v1, [2, 3, 4, 5, 6]);

    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter().enumerate()
        // 每两个元素剔除一个
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(idx, val)| val)
        .fold(0u64, |sum, acm| sum + acm);
    println!("val_ {}", val);
    
}

const T: i32 = 11;




trait PP {
    fn a(&self);
    fn b(&self);
}

trait Pilot {
    fn fly(&self);
    fn work();
}

trait Wizad {
    fn fly(&self);
    fn work();
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot")
    }

    fn work() {
        println!("Pilot working")
    }
}

impl Wizad for Human {
    fn fly(&self) {
        println!("Wizad")
    }

    fn work() {
        println!("Wizad working")
    }
}

impl Human {
    fn fly(&self) {
        println!("Human")
    }

    fn work() {
        println!("Human working")
    }
}

// impl PartialEq for Point<i32, i32> {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

impl Add for Point<i32, i32> {
    type Output = Point<i32, i32>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Counter {
    num: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.num += 1;
        // if self.num > 5 {
        //     return None
        // }
        Some(self.num)
    }
}

trait Node {
    fn move_to(&mut self, x: f32, y: f32);
    fn draw(&self);
}

struct EmptyNde {
    x: f32,
    y: f32,
}

struct TestNode {
    x: f32,
    y: f64,
}

impl Node for EmptyNde {
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self) {
        println!("EmptyNpde: x = {}, y = {}", self.x, self.y);
    }
}

impl EmptyNde {
    fn test(&self) {
        println!("test")
    }
}

#[derive(Debug, PartialEq)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>{
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn add<T>(a: T, b: T) -> T where T: std::ops::Add<Output =T> {
    a + b
}

fn draw1(x: Box<dyn Node>) {
    x.draw();
}

fn draw2(x: &dyn Node) {
    x.draw();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    
    fn new_1(x: u32, y: u32) -> Self {
        Self { width: x, height: y }
    }

    pub fn new(x: u32, y: u32) -> Rectangle {
        Rectangle { width: (x), height: (y) }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn set_width(&mut self, x: u32) {
        self.width = x
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn set_height(&mut self, y: u32) -> () {
        self.height = y
    }
    pub fn take_ownership(self) {
        println!("self {:?}", self);
    }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn one_param<'a>(p: &'a str) -> &'a str {
    if p.len() < 3 {
        p
    } else {
        "aa"
    }
}

#[derive(Debug)]
struct Test<'a> {
    temp: &'a str
}

// x、y 和返回值至少活得和 'a一样久(因为返回值要么是 x, 要么是y)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn give_ownership() -> String {
    let s1 = String::from("value");
    s1
}

fn take_ownership(s: String) -> () {
    println!("s -> {}", s)
}

fn _calc() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    fn factory() -> Box<dyn Fn(i32) -> i32> {
        let num: i32 = 5;
        Box::new(move|x| x + num)
    }
    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer);

    let a: IpAddr = "127.0.0.1".parse().expect("panic");
    println!("{:?}", a);


    let ee: Result<IpAddr, AddrParseError> = "127.0.0.1".parse();
    let ee: Result<IpAddr, AppError> = match ee {
        Ok(ipaddr) => Ok(ipaddr),
        Err(err) => Err(AppError::from(err)),
    };

    let _ = test_write_fn().unwrap();
    let _ = re_write_fn().unwrap();

    let idx = 10;
    let vec = vec![1, 2, 3];
    let c = vec.get(idx);
}

fn get_num(n: u32) -> Option<u32> {
    if n % 2 == 0 {
        return Some(2)
    }
    None
}

fn test_get_num_1(index: usize) -> Option<i32> {
    let vv = vec![1, 2, 3, 4, 5];
    let value = vv.get(index)?;
    // let value = vv.first()?;
    Some(*value)
}

fn test_get_num() -> Option<i32> {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let n = nums.first()?;
    Some(*n)
}

fn re_write_fn() -> Result<usize, Box<dyn Error>> {
    let mut f = std::fs::File::create("./h1.txt")?;
    let size = f.write("buf__".as_bytes())?;
    f.flush()?;
    Ok(size)
}

fn test_write_fn() -> Result<usize, Box<dyn Error>> {
    let mut f = File::create("./hello1.txt")?;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };
    let size = f.write("hello fifith_lession".as_bytes())?;
    // let size = match size {
    //     Ok(size) => size,
    //     Err(error) => return Err(error),
    // };
    let cc = f.flush()?;
    // match cc {
    //     Ok(_) => return Ok(size),
    //     Err(err) => return Err(err),
    // }
    Ok(size)
} 

#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: value.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(value: std::num::ParseIntError) -> Self {
        println!("invoke impl From<std::num::ParseIntError> for AppError ~");
        AppError {
            kind: String::from("parse"),
            message: value.to_string(),
        }
    }
}

impl From<AddrParseError> for AppError {
    fn from(value: AddrParseError) -> Self {
        println!("invoke impl From<AddrParseError> for AppError~");
        AppError {
            kind: "AddrParseError".to_string(),
            message: value.to_string(),
        }
    }
}

// fn main()  -> Result<(), AppError>{
//     // panic!("~~");
    
//     let mut file = File::open("./a.txt")?;
//     let mut ss = String::new();
//     file.read_to_string(&mut ss)?;
//     ss = ss.trim().parse().unwrap();
//     let _number: usize;
//     _number = ss.parse()?;
//     println!("_number: {}", _number);
//     Ok(())
// }

fn _map_or_map_err() {
    // map() 和 map_err()
    // map可以将some或ok中的值映射为另一个
    let s1 = Some("abcde");
    let s2 = Some(5);
    let trans_fn = |s: &str| -> usize {s.chars().count()};
    assert_eq!(s1.map(trans_fn), s2);

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);
    assert_eq!(o1.map(trans_fn), o2);

    // 但是如果你想要将 Err 的值进行改变
    // map就无能为力了
    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);
    
    let trans_fn2 = |s: &str| -> isize { s.parse().unwrap() };
    assert_eq!(e1.map_err(trans_fn2), e2);
}

fn _or_and() {
    let s1 = Some("some1");
    let s2 = Some("some2");

    let n: Option<&str> = None;

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("err2");

    assert_eq!(s1.or(s2), s1);
    assert_eq!(n.or(n), n);
    assert_eq!(n.or(s1), s1);

    assert_eq!(o1.or(o2), o1);
    assert_eq!(o1.or(e1), o1);

    assert_eq!(o1.and(o2), o2);
    assert_eq!(e1.and(e2), e1);

    assert_eq!(o1.and(e1), e1);
    assert_eq!(e1.and(o1), e1);
}

fn _coll() {
    use std::collections;
    let mut v1 = Vec::new();
    v1.push(1);

    let v2 = vec![1, 2, 3];
    
    // 可以通过下标获取值
    // let dos_not_exist = &v2[100];
    let does_not_exist = v2.get(100);
    assert_eq!(does_not_exist, None);

    // 存储不同类型的元素
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string().parse().unwrap()),
        IpAddr::V6("::1".to_string().parse().unwrap()),
    ];

    for ip in v {
        show_addr(ip);
    }

    let teams_list = vec![
        ("china".to_string(), 100),
        ("usa".to_string(), 10),
        ("japan".to_string(), 5),
    ];

    let mut teams_map = HashMap::new();
    // for team in &teams_list {
    //     // 注意所有权转移, insert会转移所有权
    //     teams_map.insert(&team.0, team.1);
    // }

    let mut teams_map1 = teams_list
        .into_iter()
        // .map(|x| -> (&String, i32) {(x.0, x.1)})
        .collect::<HashMap<_, _>>();

    let score = teams_map.get(&"china".to_string()).unwrap();
    println!("{:?}", score);

    // 若不存在则插入新值
    let c: String = "canada".to_string();
    let s = teams_map.entry(&c).or_insert(50);
    *s = *s + 1;

    assert_eq!(*s, 51);
    assert_eq!(*teams_map.get(&c).unwrap(), 51);

    println!("{:?}", teams_map);

}

fn _point() {
    // let a = [1, 2, 3];
    // let b = &a;

    // println!("{:p}", b);

    // // 要获取可变引用，必须先声明可变绑定
    // let mut c = vec![1, 2 ,3];
    // // 通过 &mut获取可变引用
    // let d = &mut c;
    // d.push(4);
    // println!("{:?}", d);

    // let e = &42;
    // assert_eq!(42, *e);

    // compare();

    let mut x = 10;
    let ptr_x = &mut x as *mut i32;
    let y = Box::new(20);
    let ptr_y = &*y as *const i32;

    // 原生指针操作要放在unsafe中执行
    unsafe {
        *ptr_x += *ptr_y;
    }

    // *ptr_x += *ptr_y;
    
    assert_eq!(x, 30);

    let mut a = 11;
    println!("{}, p_ {:p}", a, &a);

    let p_a = &mut a as *mut i32;

    unsafe { 
        *p_a += 1;
    };
    println!("{}, p_ {:p}", a, &a);

    let cc = Box::new(10);
    let p_cc = *cc;
}

fn _box() {
    let box_point = Box::new(Point_a{ x: 0.0, y: 0.0 });
    // 实现 Deref 后的智能指针结构体, 就可以像普通引用一样，通过 * 进行解引用
    // *(box_point.deref());
    // let unbox_point = box_point.deref();
    // let unbox_point = *unbox_point;
    let unbox_point = *box_point;
    assert_eq!(unbox_point, Point_a{ x: 0.0, y: 0.0 });

    // 1. 避免栈上数据的拷贝
    let arr = [0; 10];
    let arr_1 = arr;
    // arr和arr_1都有各自数组的所有权，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr_1.len());

    let arr = Box::new([0; 10]);
    let arr1 = &arr;
    println!("{:?}", arr.len());
    println!("{:?}", arr_1.len());

    let cc = [1, 2, 3];

    println!("stack_{:p}, heap_{:p}", &cc, &arr);

    // 2. 特征对象 在面向对象课程中分享过 特征对象

    // 3. Box::leak()
    // Box::leak() 它可以消费掉 Box 并且强制目标从内存中消失
    // 改变了变量的生命周期
    println!("{}", gen_static_str());
}

fn main() {

    // drop_fn();
    // crate::c_test::rc_1();
    println!("in main...");

    utils_lib::local_utils::local_utils_lists();
    
}

struct HasDrop1;
struct HasDrop2;

impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1");
    }
}

impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2");
    }
}

struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops");
    }
}

// #[derive(Copy)]
struct Foo_a;

impl Drop for Foo_a {
    fn drop(&mut self) {
        println!("Dropping Foo_a");
    }
}

fn drop_fn() {
    println!("drop_fn start");
    let _x = HasTwoDrops {
        one: HasDrop1,
        two: HasDrop2,
    };
    let foo = Foo_a;
    // drop(foo);
    // println!("{}", foo);
    println!("drop_fn end");
}

fn gen_static_str() -> &'static str{
    let mut s = String::new();
    s.push_str("hello world");
    let c = Box::leak(s.into_boxed_str());
    c
}

#[derive(Debug, PartialEq)]
struct Point_a {
    x: f64,
    y: f64,
}

fn compare() {

    let ref t; t = &1;
    let m; m = &1;
    let ref n: i32; n = &1;

    let ref a = 2;
    let ref b = &2;

    let r = &1;
    let &a = r;
    let a = *r;

}

fn show_addr(ip: IpAddr) {
    match ip {
        IpAddr::V4(ipv4_addr) => println!("v4: {:?}", ipv4_addr),
        IpAddr::V6(ipv6_addr) => println!("v6: {:?}", ipv6_addr),
    }
}


#[cfg(test)]
mod c_test {
    use std::{cell::{Cell, RefCell}, rc::Rc, sync::Arc, thread};


    #[test]
    pub fn rc_1() {
        
        struct Service {
            f: Rc<u8>,
        }

        let data = Rc::new(1_u8);
        let a = Service { f: data.clone() };
        let b = Service { f: data.clone() };

        println!("a.f = {}", a.f);
        println!("b.f = {}", b.f);


        // 问题: 将Rc<T>传到新线程中，会发生什么?
        // for i in 0..3 {
        //     let c = data.clone();
        //     let _h = thread::spawn(move || {
        //         println!("f = {}", c);
        //     });
        // }

    }

    #[test]
    fn arc_1() {

        struct Service {
            f: Arc<u8>,
        }

        let data = Arc::new(1_u8);
        let a = Service { f: data.clone() };
        let b = Service { f: data.clone() };

        println!("a.f = {}", a.f);
        println!("b.f = {}", *b.f);

        let mut handles = Vec::new();
        for _i in 0..=2 {
            // 创建一个对data的引用
            let c = data.clone();
            let handle = thread::spawn(move|| {
                println!("f = {}", c);
            });
            handles.push(handle);
        }
        // 等待所有线程打印完成
        for handle in handles {
            handle.join().unwrap();
        }
        // 问题: 如何通过 Arc<T>对 T 进行修改
        // let data_v1 = *data;
        
    }

    // 通过Cell 实共享引用的内部可变性

    #[test]
    fn cell_1() {
        
        let data = Cell::new(1_i32);
        // 创建多个data的共享引用
        let a = &data;
        let b = &data;

        println!("inner = {}", data.get());
        a.set(2);
        println!("inner = {}", data.get());
        b.set(3);
        println!("inner = {}", data.get());
        a.set(4);
        println!("inner = {}", data.get());
    }

    #[test]
    fn ref_cell_1() {

        struct Immutable {
            inner: RefCell<i32>,
        }
        impl Immutable {
            fn add(&self) {
                let mut f_mut = self.inner.borrow_mut();
                *f_mut += 1;
            }
        }

        let a = Immutable {
            inner: RefCell::new(0),
        };
        *a.inner.borrow_mut() += 1;
        println!("inner_1 = {}", a.inner.borrow());

        let f = a.inner.borrow_mut();
        println!("inner = {}", f);
        drop(f);

        println!("inner = {}", a.inner.borrow());
        a.add();
        a.add();
        println!("inner = {}", a.inner.borrow());

    }

    #[test]
    fn rc_refcell() {

        struct Service {
            f: Rc<RefCell<u8>>,
        }

        impl Service {
            fn add(&self) {
                let mut f = self.f.borrow_mut();
                *f += 1;
            }
        }

        let data = Rc::new(RefCell::new(1_u8));
        let a = Service { f: data.clone() };
        let b = Service { f: data.clone() };
        println!("f = {}", (*data).borrow());
        a.add();
        b.add();
        println!("f = {}", (*data).borrow());
    }

    #[test]
    fn test_drop() {
        struct A {}
        impl Drop for A {
            fn drop(&mut self) {
                todo!()
            }
        }
    }

}
