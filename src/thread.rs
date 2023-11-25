use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Once;
use std::thread;
use std::thread::JoinHandle;
use std::thread::Thread;

static INIT_POOL: Once = Once::new();

struct ThreadPool {
    tx: Option<Sender<JoinHandle<()>>>,
    rx: Option<Receiver<JoinHandle<()>>>,
}

impl ThreadPool {
    pub fn new() -> Self {
        let mut pool = Self { tx: None, rx: None };
        let (tx, rx) = mpsc::channel();
        pool.rx = Some(rx);

        // 开启一个thread
        // 该thread功能：循环创建thread，并send至channel中
        thread::spawn(move || loop {
            let join_handle: JoinHandle<()> = thread::spawn(|| {});
            tx.send(join_handle).expect("send to tx error");
        })
        .join()
        .expect("failed init sender thread");

        return pool;
    }

    // 从rx中获取一个JoinHandler<()>
    pub fn get_thread(&self) -> JoinHandle<()> {
        return self.rx.as_ref().unwrap().recv().unwrap();
    }
}
