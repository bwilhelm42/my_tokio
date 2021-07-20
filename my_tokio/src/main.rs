mod my_tokio;

use rand::{Rng, thread_rng};

fn main() {
    let my_tokio = my_tokio::MyTokio::new();

    my_tokio.spawn(async {
        let mut rng = thread_rng();
        let mut rand: u64 = rng.gen();
        rand = rand % 10;
        println!("Sleep delay on task 1: {}", rand);
        std::thread::sleep(std::time::Duration::new(rand % 10, 0));
        println!("Task 1");
    });
    my_tokio.spawn(async {
        let mut rng = thread_rng();
        let mut rand: u64 = rng.gen();
        rand = rand % 10;
        println!("Sleep delay on task 2: {}", rand);
        std::thread::sleep(std::time::Duration::new(rand, 0));
        println!("Task 2");
    });
    my_tokio.spawn(async {
        let mut rng = thread_rng();
        let mut rand: u64 = rng.gen();
        rand = rand % 10;
        println!("Sleep delay on task 3: {}", rand);
        std::thread::sleep(std::time::Duration::new(rand % 10, 0));
        println!("Task 3");
    });
    my_tokio.spawn(async {
        let mut rng = thread_rng();
        let mut rand: u64 = rng.gen();
        rand = rand % 10;
        println!("Sleep delay on task 4: {}", rand);
        std::thread::sleep(std::time::Duration::new(rand % 10, 0));
        println!("Task 4");
    });

    my_tokio.run();
}
