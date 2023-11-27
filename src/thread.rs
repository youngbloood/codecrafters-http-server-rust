use crate::handle_request;
use std::net::TcpStream;
use std::sync::atomic;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;
use std::sync::mpsc::RecvTimeoutError;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub enum MsgType {
    Connect(TcpStream),
    Close(String),
}

struct UnionThread {
    tx: Sender<MsgType>,
    handler: JoinHandle<()>,
}

pub struct ConcTcpStreamPool {
    now_index: usize,
    is_stop: atomic::AtomicBool,
    list: Vec<UnionThread>,
}

impl ConcTcpStreamPool {
    pub fn new(cap: usize) -> ConcTcpStreamPool {
        let mut conc = ConcTcpStreamPool {
            now_index: 0,
            is_stop: AtomicBool::new(false),
            list: vec![],
        };
        for _i in 0..cap {
            let (tx, rx) = channel::<MsgType>();

            let handler = thread::spawn(move || {
                let current_thread = thread::current();
                let current_thread_id = current_thread.id();
                println!("Start Thread: {:?}", current_thread_id);
                loop {
                    // 200ms timeout
                    let _ = match rx.recv_timeout(Duration::from_millis(200)) {
                        Ok(mt) => match mt {
                            MsgType::Connect(mut ts) => {
                                println!("Current Thread: {:?}", current_thread_id);
                                handle_request::handle_tcp(&mut ts)
                            }
                            MsgType::Close(cause) => {
                                println!(
                                    "Current Thread: {:?}, closed with cause: {}",
                                    current_thread_id, cause
                                );
                                break;
                            }
                        },
                        Err(e) => match e {
                            RecvTimeoutError::Timeout => continue,
                            RecvTimeoutError::Disconnected => {
                                println!(
                                    "Current Thread: {:?}, disconnected, exit!",
                                    current_thread_id,
                                );
                                break;
                            }
                        },
                    };
                }
            });

            conc.list.append(&mut vec![UnionThread { tx, handler }]);
        }

        return conc;
    }

    // dispatch a TcpStream to a thread pool
    pub fn dispatch(&mut self, ts: TcpStream) -> Result<(), SendError<MsgType>> {
        if self.is_stop.load(atomic::Ordering::Relaxed) {
            println!("pool has stopped");
            return Ok(());
        }
        let union_thread = self.list.get(self.now_index).unwrap();
        // index循环
        self.now_index += 1;
        if self.now_index >= self.list.len() {
            self.now_index = 0;
        }
        return union_thread.tx.send(MsgType::Connect(ts));
    }

    // stop all the threads
    pub fn stop(&mut self) -> Result<(), SendError<MsgType>> {
        if self.is_stop.load(atomic::Ordering::Relaxed) {
            return Ok(());
        }
        self.is_stop.store(true, atomic::Ordering::Relaxed);

        for union_thread in self.list.as_mut_slice() {
            union_thread
                .tx
                .send(MsgType::Close(String::from("stopped manual")))?;

            // TODO:
            // union_thread.handler.join();
        }

        self.list = vec![];

        return Ok(());
    }
}
