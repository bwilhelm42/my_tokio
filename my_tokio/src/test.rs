struct LaterTask<Futur> {
    expiration: Futur,
}

impl<Futur: Future<Output=()>> Future for LaterTask<Futur> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        println!("My Implementation of Poll");
        let task: Pin<&mut Futur> = unsafe { self.map_unchecked_mut(|s| &mut s.expiration) };

        match task.poll(ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => {
                println!("Ready!!");