use rand::{Rng, distributions::Uniform};
use std::os::unix::net::{UnixDatagram};
use std::time::Duration;

const SOCKET_PATH: &str = "/../../../../tmp/socket.sock";

fn main() {
    // Send one random number
    // let socket = UnixDatagram::unbound().unwrap();
    // let _ = socket.connect(SOCKET_PATH);
    // let mut rnd = rand::thread_rng();
    // let range = Uniform::new_inclusive(1, 100);
    //
    // loop {
    //     let random_number: i32 = rnd.sample(range);
    //     let _ = socket.send_to(&random_number.to_be_bytes(), SOCKET_PATH);
    //     println!("random number: {}", random_number);
    //     std::thread::sleep(Duration::from_secs(1));
    // }

    //  Send buffer of 5 random numbers
    let socket = UnixDatagram::unbound().unwrap();
    let _ = socket.connect(SOCKET_PATH);
    let mut rnd = rand::thread_rng();
    let range = Uniform::new_inclusive(-10000, 10000);
    const MAX_NUMBERS: usize = 5;
    const BUFFER_SIZE: usize = 4 * MAX_NUMBERS;
    loop {
        let mut buffer = Vec::new();
        for  _i  in 0..MAX_NUMBERS {
            let random_number: i32 = rnd.sample(range);
            println!("random_number: {}", random_number);
            buffer.push(random_number);
        }
        let buffer_as_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(buffer.as_ptr() as *const u8, buffer.len() * std::mem::size_of::<i32>())
        };
        let _ = socket.send(&buffer_as_bytes);
        println!("buffer: {:?}", buffer);
        println!("buffer_as_bytes: {:?}", buffer_as_bytes);
        std::thread::sleep(Duration::from_secs(1));
    }
}