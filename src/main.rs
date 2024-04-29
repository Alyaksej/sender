use rand::{Rng, distributions::Uniform};
use std::os::unix::net::{UnixDatagram};
use std::time::Duration;

const SOCKET_PATH: &str = "/../../../../tmp/socket.sock";

fn main() {
    let socket = UnixDatagram::unbound().unwrap();
    let _ = socket.connect(SOCKET_PATH);
    let mut rnd = rand::thread_rng();
    let range = Uniform::new_inclusive(1, 100);

    // loop {
    //     let random_number: i32 = rnd.sample(range);
    //     let _ = socket.send_to(&random_number.to_be_bytes(), SOCKET_PATH);
    //     println!("random number: {}", random_number);
    //     std::thread::sleep(Duration::from_secs(1));
    // }
    const MAX_NUMBERS: usize = 5;
    const BUFFER_SIZE: usize = 4 * MAX_NUMBERS;
    loop {
        let mut buffer = vec![0u8, (BUFFER_SIZE as i32).try_into().unwrap()];
        let random_number: i32 = rnd.sample(range);
        for  i  in 0..5 {
            buffer.append(random_number)
        }
        let _ = socket.send_to(&random_number.to_be_bytes(), SOCKET_PATH);
        println!("random number: {}", random_number);
        std::thread::sleep(Duration::from_secs(1));
    }
}