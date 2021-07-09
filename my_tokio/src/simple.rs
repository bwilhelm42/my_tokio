struct MyFut {
	future: Pin<Box<MyFuture>>,
}

impl Future for MyFut {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		println!("made it!");
		Poll::Pending
	}
}

fn say_hi() {
	println!("hello!");
}

fn main() {
	let fun_ptr: fn() = say_hi;
	let future = (async || {
		println!("hello");
	})
	let something: i32 = future;

	fun_ptr();
}