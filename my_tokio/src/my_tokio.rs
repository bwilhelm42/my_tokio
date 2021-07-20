use std::{
	sync::{Arc, Mutex, mpsc},
	pin::Pin,
	task::{Context, Poll},
	future::Future,
	thread,
};
use threadpool;
use futures::task;

type BoxedFuture = Mutex<Pin<Box<dyn Future<Output = ()> + Sync + Send>>>;

pub struct MyTokio {
	sender: mpsc::Sender<Arc<Task>>,
	receiver: mpsc::Receiver<Arc<Task>>,
}

pub struct Task {
	future: BoxedFuture,
}

impl MyTokio {
	pub fn new() -> MyTokio {
		let (tx, rx) = mpsc::channel();
		MyTokio {sender: tx, receiver: rx}
	}
	pub fn spawn<T:>(&self, fut: T)
	where T: Future<Output = ()> + Sync + Send + 'static,
	{
		let task = Arc::new(Task {
			future: Mutex::new(Box::pin(fut)),
		});
		self.sender.send(task).unwrap();
	}
	pub fn run(self) {
		let pool = threadpool::ThreadPool::new(5);
		loop {
			let task = self.receiver.recv().unwrap();
			pool.execute(move ||{
					let waker = task::noop_waker();
					let mut cx = Context::from_waker(&waker);
					let mut future = task.future.lock().unwrap();
					if future.as_mut().poll(&mut cx) == Poll::Pending {
						println!("handle this");
					}
					println!("executed on thread: {:?}", thread::current().id());
			});
		}
	}
}