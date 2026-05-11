use std::time::Duration;

fn main() {
    trpl::block_on(async {

        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(6)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }


        //         let a = async {
        //     println!("'a' started.");
        //     slow("a", 30);
        //     trpl::yield_now().await;
        //     slow("a", 10);
        //     trpl::yield_now().await;
        //     slow("a", 20);
        //     trpl::yield_now().await;
        //     println!("'a' finished.");
        // };

        // let b = async {
        //     println!("'b' started.");
        //     slow("b", 75);
        //     trpl::yield_now().await;
        //     slow("b", 10);
        //     trpl::yield_now().await;
        //     slow("b", 15);
        //     trpl::yield_now().await;
        //     slow("b", 350);
        //     trpl::yield_now().await;
        //     println!("'b' finished.");
        // };



        // trpl::select(a, b).await;
    });
}

// fn slow(name: &str, ms: u64) {
//     std::thread::sleep(Duration::from_millis(ms));
//     println!("'{name}' ran for {ms}ms");
// }

async fn timeout<F>(
    future: F,
    max_time: Duration,
) -> Result<F::Output, Duration>
where
    F: std::future::Future,
{
    match trpl::select(future, trpl::sleep(max_time)).await {
        trpl::Either::Left(result) => Ok(result),
        trpl::Either::Right(_) => Err(max_time),
    }
}