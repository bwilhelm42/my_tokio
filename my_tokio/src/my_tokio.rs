use std::{
	sync::mpsc,
	pin::Pin,
	task::{Context, Poll},
	future::Future,
};
use futures::{
	task,
	executor,
};
use rand::Rng;

type MyFuture = dyn Future<Output = ()> + 'static;

struct MyFut {
	future: Pin<Box<MyFuture>>,
}

pub struct MyTokio {
	sender: mpsc::Sender<Pin<Box<MyFuture>>>,
	receiver: mpsc::Receiver<Pin<Box<MyFuture>>>,
}

impl AsMut<MyFut> for MyFut {
	fn as_mut(&mut self) -> &mut MyFut {
		self
	}
}

impl Future for MyFut {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		if false {
			let mut new_self = *self;
			new_self.future.as_mut().poll(cx);
			return Poll::Ready(());
		}
		Poll::Pending
	}
}

impl MyTokio {
	pub fn new() -> MyTokio {
		let (tx, rx) = mpsc::channel();
		MyTokio {sender: tx, receiver: rx}
	}
	pub fn spawn<T: Future<Output = ()> + 'static>(&self, fut: T) {
		let boxed_fut = Box::pin(fut);
		self.sender.send(boxed_fut).unwrap();
	}
	pub fn run(&self) {
		let waker = task::noop_waker();
		let mut context = task::Context::from_waker(&waker);
		loop {
			let fut = self.receiver.recv().unwrap();
			let mut new = Box::pin(fut);
			let mut my_fut = Box::pin(MyFut {future: new});
			let waker = task::noop_waker();
			let mut cx = Context::from_waker(&waker);
			if my_fut.as_mut().poll(&mut cx) == Poll::Pending {
				println!("handle this");
			}
		}
	}
}