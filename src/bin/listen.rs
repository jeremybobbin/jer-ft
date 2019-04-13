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
    let mut buf: [u8; 2_000] = [0; 2_000];
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
