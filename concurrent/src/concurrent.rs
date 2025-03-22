use std::{collections, rc::Rc, sync::{self, atomic::AtomicU64, mpsc, Arc, Barrier, Condvar, Mutex, RwLock}, thread::{self, spawn, JoinHandle, Thread}, time::{Duration, Instant}};

fn _join() {
    // let mut vv: Vec<JoinHandle<()>> = vec![];
    // let mut vv = Vec::new();
    let i = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number from the spanned thread {} ", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // vv.push(i);

    i.join().unwrap();
    
    for i in 1..5 {
        println!("hi number from the main thread {} ", i);
            thread::sleep(Duration::from_millis(1));
    }
}

fn _move() {
    let v = vec![1, 2, 3];
    let t = thread::spawn(move|| {
        println!("here is a Vec {:?}", v);
    });
    t.join().unwrap();
}

fn _channel() {
    let (tx, rx) = sync::mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();

        // tx.send(Some(1)).unwrap();
    });

    println!("receive: {:?}", rx.recv().unwrap());
}

fn _single_sender() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move|| {
        let vals = vec![
            String::from("hi"),
            String::from("free"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    // 循环结束的依据： tx发送者的生命周期结束后
    for received in rx {
        println!("Got: {}", received);
    }
}

fn _multiple_send() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move|| {
        thread::sleep(Duration::from_secs(2));
        tx.send("hi from the tx").unwrap();
    });

    thread::spawn(move|| {
        thread::sleep(Duration::from_secs(3));
        tx1.send("hi from the tx1").unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn _sync_channel() {
    // bound -> 通道容量
    let (tx, rx) = mpsc::sync_channel(0);

    let handle = thread::spawn(move|| {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("received {}", rx.recv().unwrap());
    handle.join().unwrap();
}

fn _mutex() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    
    for _ in 0..100 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move|| {
            let mut num = counter.lock().unwrap();

            *num += 1;
            // *counter.lock().unwrap() += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("result: {}", *counter.lock().unwrap());
}

fn _rw_lock() {
    let rw = RwLock::new(5);

    {
        let r1 = rw.read().unwrap();
        let r2 = rw.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
        // drop(r1);
        // drop(r2);
    }

    {
        let mut w = rw.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
        // drop(w);

        // let r3 = rw.read();
        // println!("{:?}", r3);
    }
}

fn _condtion() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move|| {
        let (ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) =  &*pair;
    let mut started = lock.lock().unwrap();
    thread::sleep(Duration::from_secs(2));
    while !*started {
        println!("enter while..");
        started = cvar.wait(started).unwrap();
    }
    println!("started changed");
}

fn _condvar() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let handler = thread::spawn(move|| {
        let mut n = { *cflag.lock().unwrap() };
        let mut counter = 0;
        while counter < 3 {
            while !n {
                n = *ccond.wait(cflag.lock().unwrap()).unwrap();
            }
            {
                n = false;
                *cflag.lock().unwrap() = false;
            }
            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        thread::sleep(Duration::from_secs(1));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        cond.notify_one();
    }
    handler.join().unwrap();
    println!("{:?}", flag);
}

fn _barrier() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for i in 1..=6 {
        let b = barrier.clone();
        let handle = thread::spawn(move|| {
            println!("before work");
            thread::sleep(Duration::from_millis(100 * i));
            b.wait();
            println!("after work");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

const N_TIMES: u64 = 2_000_000;
const N_THREADS: usize = 10;

static R: AtomicU64 = sync::atomic::AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move|| {
        for _ in 0..n {
            R.fetch_add(1, sync::atomic::Ordering::Relaxed);
        }
    })
}

fn _atomic() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for handle in threads {
        handle.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(sync::atomic::Ordering::Relaxed));
    
    let elapsed = s.elapsed();
    println!("elapsed_ {:?}", elapsed);
    
    let s1 = Instant::now() - s;
    println!("s1_ {:?}", elapsed);
}

fn _send_and_sync() {
    let v = Rc::new(5);
    let v = Arc::new(5);

    let t = thread::spawn(move|| {
        println!("{:?}", v);
    });

    t.join().unwrap();
}

#[derive(Debug)]
struct SendStruct(*const u8);
unsafe impl Send for SendStruct {}

fn _send() {
    let p = SendStruct(5 as *const u8);
    let t = thread::spawn(move|| {
        println!("{:?}", p);
    });
    t.join().unwrap();
}
