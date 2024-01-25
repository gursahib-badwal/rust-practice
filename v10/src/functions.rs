use std::{sync::{Arc, Mutex, mpsc::{Receiver, self, Sender}}, thread::{JoinHandle, self, current}};

pub fn shared_state_incr(x:Arc<Mutex<i32>>) {
    // unimplemented!();
    let x_copy = Arc::clone(&x);

    let handle= thread::spawn(move ||{
        let mut current_num= x_copy.lock().unwrap();
        *current_num += 1;
    });
    handle.join().unwrap();
}

pub fn distributed_receive_incr(rec:Receiver<fn(i32) -> i32>,mut x:i32) -> i32 {
    // unimplemented!();
    for recieved_func in rec {
        x = recieved_func(x)
    }
    return x
}

pub fn distributed_send_incr(fns:Vec<fn(i32) -> i32>) -> (Vec<JoinHandle<()>>,Receiver<fn(i32) -> i32>) {
    // unimplemented!();
    let (tx,rx) = mpsc::channel();
    let mut handles_vec = vec![];

    for function in fns {

        let tramsmitter_clone = mpsc::Sender::clone(&tx);
        let join_handle = thread::spawn(move ||{
            tramsmitter_clone.send(function).unwrap(); //each transmitter sending one function
        });

        handles_vec.push(join_handle);
    }
    return (handles_vec, rx);
}
 