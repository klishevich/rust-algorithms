// use easybench::bench; did not figure out how to run it
use std::time::{SystemTime, UNIX_EPOCH};

fn euler1_unpar(input: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..input {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i as i64;
        }
    }
    sum
}

fn euler1_par(input: i64) -> i64 {
    use std::thread;

    let handle1 = thread::spawn(move || {
        let mut thread1_sum: i64 = 0;
        for i in 1..input / 2 {
            if i % 3 == 0 || i % 5 == 0 {
                thread1_sum += i as i64;
            }
        }

        thread1_sum
    });

    let handle2 = thread::spawn(move || {
        let mut thread2_sum: i64 = 0;

        for i in (input / 2)..input {
            if i % 3 == 0 || i % 5 == 0 {
                thread2_sum += i as i64;
            }
        }

        thread2_sum
    });

    handle1.join().unwrap() + handle2.join().unwrap()
}

fn main() {
    let input = 10000000000;

    let start1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let res1 = euler1_unpar(input);
    let end1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!(
        "res one thread {} duration {}",
        res1,
        (end1 - start1).as_millis()
    );

    let start2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let res2 = euler1_par(input);
    let end2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!(
        "res two threads {} duration {}",
        res2,
        (end2 - start2).as_millis()
    );
}
