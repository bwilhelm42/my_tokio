use std::{
	sync::{Arc, Mutex, mpsc},
	pin::Pin,
	task::{Context, Poll},
	future::Future,
};
use threadpool;
use futures::task;

type BoxedFuture = Mutex<Pin<Box<dyn Future<Output = ()> + Sync + Send>>>;

pub struct MyTokio {
	sender: mpsc::SyncSender<Arc<Task>>,
	receiver: mpsc::Receiver<Arc<Task>>,
}

pub struct Task {
	future: BoxedFuture,
	sender: mpsc::SyncSender<Arc<Task>>,
}

impl task::ArcWake for Task {
	fn wake_by_ref(arc_self: &Arc<Self>) {
		let clone = arc_self.clone();
		arc_self.sender.send(clone).unwrap();
	}
}

impl MyTokio {
	pub fn new() -> MyTokio {
		let (tx, rx) = mpsc::sync_channel(10);
		MyTokio {sender: tx, receiver: rx}
	}
	pub fn spawn<T:>(&self, fut: T)
	where T: Future<Output = ()> + Sync + Send + 'static,
	{
		let task = Arc::new(Task {
			future: Mutex::new(Box::pin(fut)),
			sender: self.sender.clone(),
		});
		self.sender.send(task).expect("Error");
	}
	pub fn run(self) {
		let pool = threadpool::ThreadPool::new(5);
		loop {
			let task = self.receiver.recv().unwrap();
			pool.execute(move ||{
				let mut future = task.future.lock().unwrap();
				let waker = task::waker_ref(&task);
				let mut cx = Context::from_waker(&waker);
				if let Poll::Pending = future.as_mut().poll(&mut cx) {
					println!("trying again in 5 seconds on thread {:?}", std::thread::current().id());
					std::thread::sleep(std::time::Duration::new(5, 0));
					cx.waker().wake_by_ref();
				}
			});
		}
	}
}