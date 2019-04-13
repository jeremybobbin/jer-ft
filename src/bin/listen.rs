use std::{
    fs::{
        self,
        File
    },
    net::{
        TcpListener,
        TcpStream
    },
    io::{
        self,
        Read,
        Write,
        BufReader
    },
    sync::{
        mpsc,
        Arc,
        Mutex,
    },
    thread::{
        self,
        JoinHandle
    }
};

//struct Sender {
//    tx: mpsc::Sender,
//    threads: Vec<JoinHandle>,
//    count: usize
//}
//
//impl Sender {
//    fn new(count: usize) -> Sender {
//        let threads = Vec::new();
//        let (rx, tx) = mpsc::channel();
//        let rx = Arc::new(rx);
//        for _ in 0..count {
//            let t = thread::spawn(move || {
//
//            });
//            threads.push(t);
//        }
//    }
//}
//
//impl Write for Sender {
//    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//        let middle = buf.len() / 2;
//        self.t1.send(buf[..middle]);
//        self.t2.send(buf[..middle]);
//    }
//}

fn main() {
    listen()
        .unwrap();
}

fn listen() -> io::Result<()> {

    let l1 = TcpListener::bind("192.168.0.7:3000")?;
    let l2 = TcpListener::bind("192.168.0.7:3001")?;
    let mut s1 = l1.incoming()
        .filter_map(Result::ok)
        .next()
        .unwrap();
    let mut s2 = l2.incoming()
        .filter_map(Result::ok)
        .next()
        .unwrap();

    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    let rx1 = Arc::new(Mutex::new(rx));
    let rx2 = Arc::clone(&rx1);


    let t1 = thread::spawn(move || {
        loop {
            let buf = rx1.lock().unwrap().recv().unwrap();
            s1.write_all(&buf).unwrap();
        }
    });

    let t2 = thread::spawn(move || {
        loop {
            let buf = rx2.lock().unwrap().recv().unwrap();
            s2.write_all(&buf).unwrap();
        }
    });

    let mut file = BufReader::new(File::open("big")?);
    let mut buf: [u8; 1_000_000] = [0; 1_000_000];
    println!("Entering loop");
    loop {
        let res = file.read(&mut buf)?;
        // Stream broke, break.
        if res == 0 {
            break
        }
        let mid = buf.len() / 2;
        let first = buf[..mid].to_owned();
        let second = buf[mid..].to_owned();
        tx.send(first).unwrap();
        tx.send(second).unwrap();
    }
    Ok(())
}
