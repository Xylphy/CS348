use std::{
    io::{self, Write},
    sync::{Arc, Condvar, Mutex, MutexGuard},
    thread,
};

fn read_input<T: std::str::FromStr>(prompt: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

struct SharedState {
    buffer: Vec<char>,
    written: usize,
    read: usize,
}

fn main() {
    let message: String = read_input("Enter Message: ");
    let buffer_size: usize = read_input("Enter Buffer Size: ");
    let message_length: usize = message.chars().count();

    let shared: Arc<(Mutex<SharedState>, Condvar)> = Arc::new((
        Mutex::new(SharedState {
            buffer: vec![char::default(); buffer_size],
            written: 0,
            read: 0,
        }),
        Condvar::new(),
    ));

    println!("Initiating Receiving...");
    let shared_receiver: Arc<(Mutex<SharedState>, Condvar)> = Arc::clone(&shared);
    let receiver: thread::JoinHandle<()> = thread::spawn(move || {
        let (lock, cvar): &(Mutex<SharedState>, Condvar) = &shared_receiver;

        while {
            let mut state: MutexGuard<'_, SharedState> = lock.lock().unwrap();

            while state.read == state.written && state.written < message_length {
                state = cvar.wait(state).unwrap();
            }

            if state.read < state.written {
                println!("Received: {}", state.buffer[state.read % buffer_size]);
                state.read += 1;
                drop(state);
                cvar.notify_one();
                true
            } else {
                false
            }
        } {}

        println!("Receiver Terminating...");
    });

    println!("Initiating Sending...");
    let shared_sender: Arc<(Mutex<SharedState>, Condvar)> = Arc::clone(&shared);
    let sender: thread::JoinHandle<()> = thread::spawn(move || {
        let (lock, cvar): &(Mutex<SharedState>, Condvar) = &shared_sender;

        for c in message.chars() {
            let mut state: MutexGuard<'_, SharedState> = lock.lock().unwrap();

            while state.written - state.read >= buffer_size {
                state = cvar.wait(state).unwrap();
            }

            let index: usize = state.written % buffer_size;
            state.buffer[index] = c;
            println!("Sent: {}", state.buffer[index]);
            state.written += 1;
            drop(state);
            cvar.notify_one();
        }

        println!("Sender Terminating...");
    });

    sender.join().unwrap();
    receiver.join().unwrap();

    println!("Simulation Done...");
}
