use crossbeam_utils::sync::WaitGroup;
use std::thread;

fn sample_cross_beam() {
    // Create a new wait group.
    let wg = WaitGroup::new();

    for i in 0..4 {
        // Create another reference to the wait group.
        let wg = wg.clone();

        thread::spawn(move || {
            // Do some work.
            println!("work from thread : {}", i);
            // Drop the reference to the wait group.
            drop(wg);
        });
    }

    // Block until all threads have finished their work.
    wg.wait();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        sample_cross_beam();
    }
}
