use std::sync::mpsc::sync_channel;
use std::thread;

static NTHREADS: i32 = 3;

#[derive(Debug)]
struct Context {
    id: i32,
}

fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    //let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let (tx, rx) = sync_channel(NTHREADS as usize);
    let mut children = Vec::with_capacity(NTHREADS as usize);

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        let cxt = Context{
            id:id,
        };

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(cxt).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {        
        let received = rx.recv(); // blocks the current thread if there are no messages available
        ids.push(received);
    }
    
    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
}
