use anyhow::{Result, Ok, anyhow};
use futures::future::{BoxFuture, join_all, select_all, select_ok, try_join_all};
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
    /// -- Select --
    /// Selects run a bunch of futures concurrently
    /// and give you control as soon as the first
    /// one is completed. If you want the rest to
    /// be completed/polled, you need to call Select
    /// again (e.g. recursively)


    println!("---- select_all_example");
    let futures1 = prepare_futures();
    select_all_example(futures1).await?;

    println!("----- select_all_example_recursive");
    let futures2 = prepare_futures();
    select_all_example_recursive(futures2).await?;

    println!("----- select_ok_example");
    let futures3 = prepare_futures();
    select_ok_example(futures3).await?;

    println!("----- select_ok_example");
    let futures4 = prepare_futures();
    select_ok_example_recursive(futures4).await?;


    /// -- Joins --
    /// Joins run a bunch of futures concurrently
    /// and give you control as soon as all of the
    /// are  completed.

    println!("----- try_join_all_example");
    let futures5 = prepare_futures();
    try_join_all_example(futures5).await?;

    println!("----- join_all_example");
    let futures5 = prepare_futures();
    join_all_example(futures5).await?;

    /// -- select OK -- special case
    /// first OK: stop, fail: try recursively
    //let futures6 = prepare_futures_failable();
    //select_ok_example_ok_stop_fail_continue_recursive(futures6);

    Ok(())
}

/// As soon as the the first function is Ok; stop and ignore the rest.
/// If it's not OK; continue recursively
/*#[async_recursion]
async fn select_ok_example_ok_stop_fail_continue_recursive(futures: Vec<BoxFuture<'static, Result<u32>>>)
                                                           -> Result<(u32, Vec<BoxFuture<'static, Result<u32>>>)> {
    let resp = select_ok(futures).await;
    let (item_resolved, remaining) = resp.unwrap();
    if resp.is_ok() {
        // it should stop; so we return remaining=empty
        let ok_result = (item_resolved, Vec::new());
        Ok(ok_result)
    } else {
        if !remaining.is_empty() {
            let resp_to_recurse = select_ok_example_ok_stop_fail_continue_recursive(remaining).await;
            if resp_to_recurse.is_ok() {
                let (ok_value, remaining) = resp.unwrap();
                let ok_result = (ok_value, Vec::new());
                Ok(ok_result)
            } else {
                let (ok_value, remaining) = resp.unwrap();
            }
        } else {
            Err(anyhow!("all shit! all failed"))
        }
    }
}
*/
async fn try_join_all_example(futures: Vec<BoxFuture<'static, Result<u32>>>) -> Result<()> {
    let r = try_join_all(futures).await;
    Ok(())
}

async fn join_all_example(futures: Vec<BoxFuture<'static, Result<u32>>>) -> Result<()> {
    let r = join_all(futures).await;
    Ok(())
}

#[async_recursion]
async fn select_ok_example_recursive(futures: Vec<BoxFuture<'static, Result<u32>>>)
                                     -> Result<Vec<BoxFuture<'static, Result<u32>>>> {
    let resp = select_ok(futures).await;
    if resp.is_ok() {
        let (item_resolved, remaining) = resp.unwrap();
        if remaining.is_empty() {
            let no_remaining: Vec<BoxFuture<Result<u32>>> = Vec::new();
            Ok(no_remaining)
        } else {
            select_ok_example_recursive(remaining).await
        }
    } else {
        Err(anyhow!("Error!!!!"))
    }
}

async fn select_ok_example(futures: Vec<BoxFuture<'static, Result<u32>>>) -> Result<()> {
    let resp = select_ok(futures).await;
    match resp {
        Result::Ok((item_resolved, remaining_futures)) => {
            // do nothing
        }
        Err(_) => { println!("Something goes wrong!!") }
    }
    Ok(())
}


async fn select_all_example(futures: Vec<BoxFuture<'static, Result<u32>>>) -> Result<()> {
    let (item_resolved, ready_future_index, _remaining_futures) =
        select_all(futures).await;
    Ok(())
}

#[async_recursion]
async fn select_all_example_recursive(futures: Vec<BoxFuture<'static, Result<u32>>>)
                                      -> Result<Vec<BoxFuture<'static, Result<u32>>>> {
    let (resolved, future_index, remaining) = select_all(futures).await;
    if remaining.is_empty() {
        let no_remaining: Vec<BoxFuture<Result<u32>>> = Vec::new();
        Ok(no_remaining)
    } else {
        select_all_example_recursive(remaining).await
    }
}

fn prepare_futures() -> Vec<BoxFuture<'static, Result<u32>>> {
    vec![
        Box::pin(get_async_task(1, 5).boxed()),
        Box::pin(get_async_task(2, 4).boxed()),
        Box::pin(get_async_task(3, 1).boxed()),
        Box::pin(get_async_task(4, 2).boxed()),
        Box::pin(get_async_task(5, 3).boxed()),
    ]
}

async fn get_async_task(task_id: u32, seconds: u64) -> Result<u32> {
    println!("starting {}", task_id);
    let duration = Duration::new(seconds, 0);

    sleep(duration).await;

    println!("task {} complete!", task_id);
    Ok(task_id)
}


fn prepare_futures_failable() -> Vec<BoxFuture<'static, Result<u32>>> {
    vec![
        Box::pin(get_async_task_failable(1, 5).boxed()),
        Box::pin(get_async_task_failable(2, 4).boxed()),
        Box::pin(get_async_task_failable(3, 1).boxed()),
        Box::pin(get_async_task_failable(4, 2).boxed()),
        Box::pin(get_async_task_failable(5, 3).boxed()),
    ]
}

async fn get_async_task_failable(task_id: u32, seconds: u64) -> Result<u32> {
    println!("starting {}", task_id);
    let duration = Duration::new(seconds, 0);

    sleep(duration).await;

    println!("task {} complete!", task_id);
    if task_id == 4 {
        Ok(task_id)
    } else {
        Err(anyhow!("Failed like there is no tomorrow task_id={:?}", task_id))
    }
}