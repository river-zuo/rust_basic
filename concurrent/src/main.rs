use std::future::Future;

use futures::{channel::oneshot, executor::block_on, stream};

mod concurrent;

mod _block_on {

    async fn hello_world() {
        println!("hello, world");
    }
    fn block_on() {
        let f = hello_world();
        futures::executor::block_on(f);
    }
}

async fn hello_world() {
    let hc = hello_cat();
    hc.await;
    println!("hello_world");
}

async fn hello_cat() {
    println!("hello, cat");
}

fn _await() {
    let f = hello_world();
    block_on(f);
}

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "bird".to_string(),
        name: String::from("ji ji zha zha"),
    }
}

async fn sing_song(song: Song) {
    println!("I like a {}: {} {}", song.author, song.name, "ji ji zha zha")
}

async fn sing_and_song() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn dance() {
    println!("dancing")
}

async fn async_main() {
    let f1 = sing_and_song();
    let f2 = dance();
    futures::join!(f1, f2, dance());
}

fn _block_join() {
    // block_on(sing_song(song));
    // block_on(dance());
    // let song = learn_song().await;
    futures::executor::block_on(async_main());
}

// fn bad() -> impl Future<Output = u8> {
//     let x = 5;
//     borrow_x(&x);
// }

fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

async fn borrow_x(x: &u8) -> u8 {
    *x
}

fn move_back() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        println!("{}", my_string);
    }
}

use tokio;

#[tokio::main]
async fn main() {
    
    // block_on(good());
    // block_on(bad());
    // move_back();
    // block_on(move_back());
    // dance().await;

    use futures::channel::oneshot;
    use futures::{self, StreamExt, TryStreamExt};

    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    let (_tx3, rx3) = oneshot::channel();
    
    let streams = stream::iter(vec![rx1, rx2, rx3]);
    
    let fut = streams.map(Ok).try_for_each_concurrent(2, |rx| async move {
        let res: Result<(), oneshot::Canceled>  = rx.await;
        res
    });
    
    tx1.send(()).unwrap();
    drop(tx2);

    assert_eq!(Err(oneshot::Canceled), fut.await);
}

// 自引用
#[cfg(test)]
mod self_ref {

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
            }
        }

        fn init(&mut self) {
            let self_ref: *const String = &self.a;
            self.b = self_ref;
        }

        fn a(&self) -> &str {
            &self.a
        }

        fn b(&self) -> &String {
            unsafe {
                &*(self.b)
            }
        }
    }

    #[test]
    fn t0() {
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

        let mut test1 = Test::new("test1");
        let mut test2 = Test::new("test2");
        test1.init();
        test2.init();

        println!("a: {}, b: {}", test1.a(), test1.b());
        println!("a: {}, b: {}", test2.a(), test2.b());
        std::mem::swap(&mut test1, &mut test2);
        println!("swap_ a: {}, b: {}", test1.a(), test1.b());
        println!("swap_ a: {}, b: {}", test2.a(), test2.b());

    }
}

#[cfg(test)]
mod unpin {
    use std::{marker::PhantomPinned, pin::Pin};

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现`!Unpin
            }
        }

        fn init(self: Pin<&mut Self>) {
            // let self_ref: *const String = &self.a;
            // self.b = self_ref;
            let self_ptr: *const String = &self.a;
            let this = unsafe {
                self.get_unchecked_mut()    
            };
            this.b = self_ptr;
        }

        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }

        fn b(self: Pin<&Self>) -> &String {
            unsafe {
                &*(self.b)
            }
        }
    }
    
    #[test]
    fn t0() {
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

        let mut test1 = Test::new("test1");
        let mut test1 = unsafe {
            Pin::new_unchecked(&mut test1)
        };
        
        let mut test2 = Test::new("test2");
        let mut test2 = unsafe {
            Pin::new_unchecked(&mut test2)
        };

        test1.init();
        test2.init();

        // println!("a: {}, b: {}", test1.a(), test1.b());
        // println!("a: {}, b: {}", test2.a(), test2.b());
        // std::mem::swap(&mut test1, &mut test2);
        // println!("swap_ a: {}, b: {}", test1.a(), test1.b());
        // println!("swap_ a: {}, b: {}", test2.a(), test2.b());

        // println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
        // println!("a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));
        // std::mem::swap(test1.get_mut(), test2.get_mut()); // error here
        // println!("swap_ a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
        // println!("swap_ a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));
    }
}

#[cfg(test)]
mod test_councurrent {
    use std::sync::{Arc, Mutex};

    #[test]
    fn test0() {
        let m = Arc::new(Mutex::new(false));
        let rm = m.clone();
        // let re = m.clone().lock().unwrap();
        // let re = rm.lock().unwrap();
        // println!("{:?}", re);

        let try_lock = m.try_lock().unwrap();
        println!("{:?}", try_lock);
    }

}

#[cfg(test)]
 mod what_is_future {
    use futures::executor::block_on;

    async fn hello_word() -> u32 {
        println!("hello, world");
        1
    }

    #[test]
    fn t0() {
        let future = hello_word();
        block_on(future);
    }
 }

