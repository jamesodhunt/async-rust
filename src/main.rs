use anyhow::{anyhow, Result};
use futures::future::join_all;
use rand::Rng;
use std::env;
use std::process::exit;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::{join, select};

fn get_random_sleep_time() -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(1..100)
}

//--------------------------------------------------------------------
// A set of functions that sleep for a random amount of time

async fn async_foo() -> Result<()> {
    eprintln!("INFO: START: foo");

    let ms = get_random_sleep_time();

    tokio::time::sleep(Duration::from_millis(ms)).await;

    eprintln!("INFO: END: foo");

    Ok(())
}

async fn async_bar() -> Result<()> {
    eprintln!("INFO: START: bar");

    let ms = get_random_sleep_time();

    tokio::time::sleep(Duration::from_millis(ms)).await;

    eprintln!("INFO: END: bar");

    Ok(())
}

async fn async_baz() -> Result<()> {
    eprintln!("INFO: START: baz");

    let ms = get_random_sleep_time();

    tokio::time::sleep(Duration::from_millis(ms)).await;

    eprintln!("INFO: END: baz");

    Ok(())
}

async fn async_slow() -> Result<()> {
    eprintln!("INFO: START: async_slow");

    let secs = 3;

    // A long (asynchronous) sleep.
    tokio::time::sleep(Duration::from_secs(secs)).await;

    eprintln!("INFO: END: async_slow");

    Ok(())
}

//--------------------------------------------------------------------

async fn async_run_blocking_code() -> Result<()> {
    eprintln!("INFO: START: async_run_blocking_code");

    let secs = 3;

    // XXX: BUG: This runs a _synchronous_ sleep call that blocks the runtime.
    std::thread::sleep(Duration::from_secs(secs));

    eprintln!("INFO: END: async_run_blocking_code");

    Ok(())
}

async fn funcs_not_run() -> Result<()> {
    eprintln!("INFO: funcs_not_run:\n");

    let _ = async_foo();
    let _ = async_bar();
    let _ = async_baz();
    let _ = async_slow();

    Ok(())
}

async fn wait_for_1st_func_to_finish() -> Result<()> {
    eprintln!("INFO: wait_for_1st_func_to_finish:\n");

    let task_foo = async_foo();
    let task_bar = async_bar();
    let task_baz = async_baz();
    let task_slow = async_slow();

    select! {
        _ = task_foo => { println!("INFO: foo finished 1st")},
        _ = task_bar => { println!("INFO: bar finished 1st")},
        _ = task_baz => { println!("INFO: baz finished 1st")},
        _ = task_slow => { println!("INFO: slow task finished 1st")},
    };

    Ok(())
}

async fn wait_for_all_funcs_to_finish() -> Result<()> {
    eprintln!("INFO: wait_for_all_funcs_to_finish:\n");

    let task_foo = async_foo();
    let task_bar = async_bar();
    let task_baz = async_baz();
    let task_slow = async_slow();

    // Wait for all the tasks to finish
    let (foo_result, bar_result, baz_result, slow_result) =
        join!(task_foo, task_bar, task_baz, task_slow);

    // Check the result of all the tasks
    let _ = foo_result?;
    let _ = bar_result?;
    let _ = baz_result?;
    let _ = slow_result?;

    Ok(())
}

async fn run_one_at_at_time() -> Result<()> {
    eprintln!("INFO: run_one_at_at_time:\n");

    let task_foo = async_foo();
    let task_bar = async_bar();
    let task_baz = async_baz();
    let task_slow = async_slow();

    let _ = task_bar.await;
    let _ = task_baz.await;
    let _ = task_foo.await;
    let _ = task_slow.await;

    Ok(())
}

async fn block_runtime() -> Result<()> {
    eprintln!("INFO: block_runtime:\n");

    let task_foo = async_foo();
    let task_bar = async_bar();
    let task_baz = async_baz();
    let task_blocker = async_run_blocking_code();

    // XXX: The blocker task will always finish first as it is seemingly
    // always the first task scheduled after the blocking sleep finishes.
    select! {
        _ = task_foo => { println!("INFO: foo finished 1st")},
        _ = task_bar => { println!("INFO: bar finished 1st")},
        _ = task_baz => { println!("INFO: baz finished 1st")},
        _ = task_blocker => { println!("INFO: blocker task finished 1st")},
    };

    Ok(())
}

async fn spawn_tasks_and_wait() -> Result<()> {
    eprintln!("INFO: spawn_tasks_and_wait:\n");

    let mut tasks: Vec<JoinHandle<Result<()>>> = vec![];

    // Start some async tasks
    for _ in 0..10 {
        let handle = tokio::spawn(async_foo());

        tasks.push(handle);
    }

    // Wait for all threads to finish
    let results = join_all(tasks).await;

    // Check if any of the tasks failed
    for result in results {
        let _ = result?;
    }

    Ok(())
}

//--------------------------------------------------------------------

async fn real_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let cmd = args.get(1).ok_or("need command").map_err(|e| anyhow!(e))?;

    match cmd.as_str() {
        "block-runtime" => block_runtime().await,
        "not-run" => funcs_not_run().await,
        "one-at-a-time" => run_one_at_at_time().await,
        "spawn-and-wait" => spawn_tasks_and_wait().await,
        "wait-for-1st" => wait_for_1st_func_to_finish().await,
        "wait-for-all" => wait_for_all_funcs_to_finish().await,

        _ => Err(anyhow!("unknown command")),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_name = &args[0];

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    // Block on allows a synchronous function to run an async function
    // and wait for it to finish.
    let result = rt.block_on(real_main());

    if let Err(e) = result {
        eprintln!("ERROR: {}: {:#?}", program_name, e);
        exit(1);
    }

    result
}
