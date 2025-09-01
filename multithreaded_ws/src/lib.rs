use std::{
  sync::{mpsc, Arc, Mutex},
  thread::{self, Thread},
};

/// A simple thread pool
pub struct ThreadPool {
  // Vector to hold worker threads
  workers: Vec<Worker>,
  // Sender to submit jobs to the worker threads
  sender: mpsc::Sender<Job>,
}

/// Represents a job to be executed by a worker
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Represents a single worker thread
pub struct Worker {
  // Unique ID for the worker
  id: i32,
  // The actual thread handle
  thread: thread::JoinHandle<()>,
}

impl Worker {
  /// Create a new Worker with a given ID
  pub fn new(id: i32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
      // Spawn a thread for this worker
      // Currently, the closure does nothing with the receiver
      let thread = thread::spawn(move || {
        loop {
          let job = receiver.lock().unwrap().recv().unwrap();

          println!("Worker {id} got a job; executing.");

          job();
      }
      });

      // Construct and return the Worker struct
      Worker { id, thread }
  }
}

impl ThreadPool {
  /// Create a new ThreadPool with the given number of threads
  ///
  /// # Panics
  ///
  /// Will panic if the size is zero
  pub fn new(size: usize) -> ThreadPool {
      // Ensure the pool has at least one thread
      assert!(size > 0);

      // Create a channel to send jobs to workers
      let (sender, receiver) = mpsc::channel();

      // Wrap the receiver in Arc<Mutex<>> so multiple threads can share it safely
      let receiver = Arc::new(Mutex::new(receiver));

      // Pre-allocate the vector for worker threads
      let mut workers = Vec::with_capacity(size);

      // Create each worker and push it to the vector
      for id in 0..size {
          // Clone the Arc so each thread gets shared ownership of the same receiver
          workers.push(Worker::new(id as i32, Arc::clone(&receiver)));
      }

      // Return the ThreadPool struct
      ThreadPool { workers, sender }
  }



  pub fn execute <F>(&self, f: F)
  where 
  F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
}
