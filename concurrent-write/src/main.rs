// Problem : https://stackoverflow.com/q/69724691/1087681
// How to collect multiple results from concurrently working loop?

use futures::{
    stream, StreamExt,
    lock::Mutex,
};
use std::{
    time::Duration,
    thread,
    sync::Arc,
    time::Instant,
};

#[tokio::main]
async fn main() {
    let inputs = stream::iter(0..=1000);
    let before = Instant::now();
// This block works
//     {
//         inputs
//             .for_each_concurrent(1024, |input| test1(input))
//             .await;
//     }

// This block also works
// inputs
//     .for_each_concurrent(1024, |input| async move {
//         let result = second_try(input).await;
//             if result > 0 {
//                 println!("{}", result)
//             }
//     }).await;

// Final solution
    let output_values = Arc::new(Mutex::new(Vec::new()));
    inputs
        .for_each_concurrent(1024, |input| {
            let output_values = output_values.clone();

            async move {
                let result = test_with_retval(input).await;
                if result > 0 {
                    output_values.lock().await.push(result);
                }
            }
        })
        .await;

    output_values.lock().await.iter().for_each(|x| println!("{:?}", x));

    println!("Elapsed time: {:.2?}", before.elapsed());
}

async fn test(input: u16) {
    thread::sleep(Duration::from_millis(200));
    if input%10 == 0 {
        println!("{}", input)
    }
}

async fn test_with_retval(input: u16) -> u16{
    thread::sleep(Duration::from_millis(200));
    if input%10 == 0 {
        return input;
    }
    0
}