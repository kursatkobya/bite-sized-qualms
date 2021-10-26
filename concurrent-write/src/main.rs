use futures::{stream, StreamExt};
use std::{
    time::Duration,
    thread,
};

#[tokio::main]
async fn main() {
    let inputs = stream::iter(0..=u16::MAX);
// This block works
//     {
//         inputs
//             .for_each_concurrent(1024, |input| test1(input))
//             .await;
//     }

// This block also works
//     inputs
//         .for_each_concurrent(1024, |input| async move {
//             let result = second_try(input).await;
//                 if result > 0 {
//                     println!("{}", result)
//                 }
//         }).await;

// Finaly the one below does not work
    let mut output_values:Vec<u16> = Vec::new();
    inputs
        .for_each_concurrent(1024, |input| async move {
            let result = second_try(input).await;
                if result > 0 {
                    output_values.push(result);
                }
        }).await;

}

async fn test1(input: u16) {
    thread::sleep(Duration::from_millis(200));
    if input%10 == 0 {
        println!("{}", input)
    }
}

async fn second_try(input: u16)-> u16{
    thread::sleep(Duration::from_millis(200));
    if input%10 == 0 {
        return input;
    }
    0
}