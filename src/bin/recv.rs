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
    recv()
        .unwrap();
}

fn recv() -> io::Result<()> {

    let mut s1 = TcpStream::connect("192.168.0.7:3000")?;
    let mut s2 = TcpStream::connect("192.168.0.7:3001")?;

    let file = File::create("copy")?;
    let mut file1 = Arc::new(Mutex::new(file));
    let mut file2 = Arc::clone(&file1);

    // let mut buf: [u8; 1_000_000] = [0; 1_000_000];
    let t1 = thread::spawn(move || {
        loop {
            let mut buf: [u8; 1_000] = [0; 1_000];
            let mut file = file1.lock().unwrap();
            let res = s1.read(&mut buf);
            if let Ok(0) = res {
                break
            }
            file.write(&mut buf);
        }
    });

    let t2 = thread::spawn(move || {
        loop {
            let mut buf: [u8; 1_000] = [0; 1_000];
            let mut file = file2.lock().unwrap();
            let res = s2.read(&mut buf);
            if let Ok(0) = res {
                break
            }
            file.write(&mut buf);
        }
    });
    t1.join();
    t2.join();
    Ok(())
}
