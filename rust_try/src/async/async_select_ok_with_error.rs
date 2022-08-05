use anyhow::{Result, Ok, anyhow};
use futures::future::{BoxFuture, select_all, select_ok};
use futures::FutureExt;
use tokio::time::{sleep, Duration};
use async_recursion::async_recursion;


/// Source code online: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018
/// StackOverflow: https://stackoverflow.com/questions/63715918/how-to-race-a-collection-of-futures-in-rust
///`select_all` expects the Futures iterable to implement UnPin, so we use `boxed` here to
/// allocate on the heap:
/// https://users.rust-lang.org/t/the-trait-unpin-is-not-implemented-for-genfuture-error-when-using-join-all/23612/3
/// https://docs.rs/futures/0.3.5/futures/future/trait.FutureExt.html#method.boxed

pub async fn run() -> Result<()> {

    println!("All tasks failing except one...");
    let futures_to_fail = prepare_futures_all_failing_except_one();
    let task_id= select_ok(futures_to_fail).await?;
    println!("Successful task: {:?}", task_id.0);

    println!("\n\n\nAll tasks failing...");
    let futures_to_fail = prepare_futures_all_failing();
    let resp = select_ok(futures_to_fail).await;
    if resp.is_err() {
        println!("All tasks failed");
    }

    Ok(())
}


fn prepare_futures_all_failing() -> Vec<BoxFuture<'static, Result<u32>>> {
    vec![
        Box::pin(get_async_task_to_fail(1, 3).boxed()),
        Box::pin(get_async_task_to_fail(2, 1).boxed()),
        Box::pin(get_async_task_to_fail(3, 2).boxed()),
    ]
}

async fn get_async_task_to_fail(task_id: u32, seconds: u64) -> Result<u32> {
    println!("starting {}", task_id);
    sleep(Duration::new(seconds, 0)).await;
    println!("Task {:?} failing", task_id);
    Err(anyhow!("Task {:?} failed", task_id))
}


fn prepare_futures_all_failing_except_one() -> Vec<BoxFuture<'static, Result<u32>>> {
    let e = vec![
        Box::pin(get_async_task_to_fail_except_one(1, 3).boxed()),
        Box::pin(get_async_task_to_fail_except_one(2, 1).boxed()),
        Box::pin(get_async_task_to_fail_except_one(3, 2).boxed()),
    ]
    e
}

async fn get_async_task_to_fail_except_one(task_id: u32, seconds: u64) -> Result<u32> {
    println!("starting {}", task_id);
    sleep(Duration::new(seconds, 0)).await;
    if task_id == 3 {
        Ok(task_id)
    } else {
        println!("Task {:?} failing", task_id);
        Err(anyhow!("Task {:?} failed", task_id))
    }
}