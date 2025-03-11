---
title: Threads and async rust
subtitle: (Originally presented on 2022-10-07)
author: James O. D. Hunt
date: 2025-03-11
---

## Intro

Grab a coffee or a pillow (your choice! ;)

Ready? Let's go...!

## Overview

- Overview of "async" (asynchronous) rust code.

- But, before we do _that_, we need to talk about threads and
  closures.

## Intro

- Rust code is _synchronous_ by default:
  ```rust
  func_a();
  func_b(); // This won't run until func_a() has completed.
  ```

- Async strategies allow you to run multiple tasks in parallel:
  - Threads
  - Async rust code

- Why should I care about async rust?
  - Makes better use of available resources.
  - Allows your code to be made "scaleable" with relatively little effort.
  - It's very fast!
  - The following Kata Containers components are written in async rust:
    - [agent](https://github.com/kata-containers/kata-containers/tree/main/src/agent)
    - [`runtime-rs`](https://github.com/kata-containers/kata-containers/tree/main/src/runtime-rs)
    - [`Dragonball`](https://github.com/kata-containers/kata-containers/tree/main/src/dragonball)

## Closures

Before looking at threads and async code, we need to look at closures...

- A rust _closure_ is like an anonymous function that "captures" variables.
- It has access to all variables in the scope in which it is called.
- By default, closure variables are captured _by reference_.
- You can recognise a closure by the pair of pipe characters: `||`.

## Closures (2)

Some examples of closures:

```rust
// closure that returns a value implicitly.
// Remember: a functions return value is the last statement in the function!
let c1 = || 7;

// as above, but with braces around the body
let c2 = || { 7 };

// closure that prints a message
let c3 = || println!("hello");

// as above, but with braces around the body
// Note: Technically, this closure returns '()'.
let c4 = || { println!("hello"); };

// closure with no args and no return value
let c5 = || { println!("hello"); }

// closure with 1 arg and no return value
let c6 = |value| { println!("{value}"); }
```

## Closures (3)

```rust
// closures with no args and a return value
let c7 = || -> u64 { println!("hello"); 42 };
let c8 = || -> Result<()> { println!("hello"); Ok(()) };

// closure with 2 args and a return type
let c9 = |s, n| -> Result<()> { println!("s: {s}, n: {n}"); Ok(()) };

// closure with 2 explicitly typed args and a return type
let c10 = |s: &str, n: u64| -> Result<()> { println!("string: {s}, num: {n}"); Ok(()) };
```

## Closures: move

Add the `move` keyword to capture closure variables _by value_.

```rust
let msg = "foo".to_string();

let handle = thread::spawn(move || -> Result<()> {
    println!("Message in thread: {msg:?}");

    Ok(())
};

// ERROR: BUG: This will not compile as the closure in the thread now owns "msg"!
println!("Message outside thread: {msg:?}");
```

> **Notes:**
>
> - The closure has access to all visible variables in its scope.
>
> - Once called, the closure **owns** the `msg` variable
>   (even though it was not passed in to the closure as a parameter!)

## Closures: move (2)

```rust
let msg = "foo".to_string();

let msg_for_thread = msg.clone();

let handle = thread::spawn(move || -> Result<()> {
    println!("Message in thread: {msg_for_thread:?}");

    Ok(())
};

// XXX: Ok! The thread has it's own copy of the variable.
println!("Message outside thread: {msg:?}");
```

> **Note:**
>
> - This is a contrived example!
> - Rather than `move`-ing the string into the closure, you could simply reference it.

## Closures: summary

<table border=1>
<tr bgcolor="gainsboro">
    <th>Closure</th>
    <th>Equivalent function</th>
    <th>Params</th>
    <th>Return value</th>
    <th>Description</th>
</tr>

<tr>
    <td><pre>|| println!("hello");</pre></td>
    <td><pre>fn name() { println!("hello"); }</pre></td>
    <td>no</td>
    <td>none</td>
    <td></td>
</tr>

<tr>
    <td><pre>|| 7</pre></td>
    <td><pre>fn name() { 7 }</pre></td>
    <td>no</td>
    <td>number</td>
    <td>Implicit return type</td>
</tr>

<tr>
    <td><pre>|| "foo".to_string()</pre></td>
    <td><pre>fn name() { "foo".to_string() }</pre></td>
    <td>no</td>
    <td>`String`</td>
    <td>Implicit return type</td>
</tr>

<tr>
    <td><pre>|| -> usize { 1234567890 } </pre></td>
    <td><pre>fn name() -> usize { 1234567890 }</pre></td>
    <td>no</td>
    <td>number</td>
    <td>Explicit return type<br>Note the required braces now!</br></td>
</tr>
<tr>
    <td><pre>|s| println!("{s}");</pre></td>
    <td>
    <pre>
    fn name<D>(s: D)
    where
        D: Display,
    {
        println!("{s}");
    }
    </pre>
    </td>
    <td>one</td>
    <td>none</td>
    <td>Implicit parameter type</td>
</tr>

<tr>
    <td><pre>|s: &str| println!("{s}");</pre></td>
    <td><pre>fn name(s: &str) { println!("{s}"); }</pre></td>
    <td>`&str`</td>
    <td>none</td>
    <td>Explicit parameter type</td>
</tr>

<tr>
    <td><pre>|s: String| -> usize { s.len() };</pre></td>
    <td><pre>fn name(s: String) -> usize { s.len(); }</pre></td>
    <td>`String`</td>
    <td>`usize`</td>
    <td>Explicit parameter and return types</td>
</tr>

<tr>
    <td>
    <pre>
    |s: &str| -> Result<()> {
        println!("{s}");

        Ok(())
    };
    </pre>
    </td>

    <td>
    <pre>
    fn name(s: &str) -> Result<()> {
        println!("{s}");

        Ok(())
    }
    </pre>
    </td>
    <td>`&str`</td>
    <td>`Result`</td>
    <td>Explicit parameter and return types</td>
</tr>
</table>

## Threads: Create a thread

- A thread is _spawned_ like this:

  ```rust
  let handle = thread::spawn(some_function));
  ```

- **Note:** `spawn()` returns a `JoinHandle` type.

- Each thread needs to be _waited on_ to ensure it has finished:

  ```rust
  handle.join();
  ```

## Threads: Simple threads example

```rust
use anyhow::{anyhow, Context, Result};
use thread::sleep;

fn run_thread() {
    sleep(Duration::from_millis(1));
}

fn real_main() -> Result<()> {
    let mut threads = vec![];

    // Start some threads
    for _ in 0..10 {
        let handle = thread::spawn(run_thread);

        threads.push(handle);
    }

    // Wait for threads to finish
    for child in threads {
        child
            .join()
            .map_err(|e| anyhow!("{e:?}")) // Convert JoinHandle into anyhow error
            .context("join failed")?;      // Add extra detail and return on error
    }

    Ok(())
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {e:#?}");
        exit(1);
    }
}
```

## Threads: Create a thread (2)

- Rather than passing a function to `spawn` like this:

  ```rust
  let handle = thread::spawn(run_thread));
  handle.join();
  ```

- ... we could alternatively pass a _closure_:

  ```rust
  use thread::sleep;

  let handle = thread::spawn(|| sleep(Duration::from_millis(1)));
  handle.join();
  ```

- We could even do this:

  ```rust
  use thread::sleep;

  // Assign our closure (anonymous function) to a variable
  let thread_closure = || { sleep(Duration::from_millis(1)) };

  let handle = thread::spawn(thread_closure);
  handle.join();
  ```

## Threads: Thread function that returns a value

```rust
use anyhow::{anyhow, Context, Result};
use thread::sleep;

fn run_thread_and_return_value() -> Result<()> {
    sleep(Duration::from_millis(1));

    Ok(())
}

fn real_main() -> Result<()> {
    let mut threads = vec![];

    // Start some threads
    for _ in 0..10 {
        threads.push(thread::spawn(run_thread_and_return_value));
    }

    // Wait for threads to finish
    for child in threads {
        child
            .join()
            .map_err(|e| anyhow!("{e:?}"))        // } XXX: Look at these
            .context("join failed")?              // } XXX: lines carefully!
            .context("thread function failed")?;  // } XXX: Do they make sense?
    }

    Ok(())
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {e:#?}");
        exit(1);
    }
}
```

## Threads: Create a thread and return a value

Rather than passing a function to `spawn` like this:

```rust
let handle = thread::spawn(run_thread_and_return_value));
handle.join();
```

... we could again alternatively pass a _closure_:

```rust
use thread::sleep;

let handle = thread::spawn(|| -> Result<()> {
    sleep(Duration::from_millis(1)));
    Ok(())
};

handle.join();
```

Compare this to how we define the previous closure (where we didn't
return a value):

```rust
let handle = thread::spawn(|| sleep(Duration::from_millis(1)));
handle.join();
```

## Threads: summary

- Create a new thread by spawning it.
- A thread must be waited on to ensure it has finished.
- Threads run _as soon as they are created_.
- You cannot rely on the order threads run in (effectively random).
- Common to run a closure in a thread, rather than a normal function.
- Pros:
  - Lighter weight than processes.
  - Allow work to run in in parallel.
- Cons:
  - Hard to synchronise and schedule data access.
  - Still not as lightweight as they could be.

## Async: How to use async rust

- You need an async _runtime_.
- The most popular runtime is [Tokio](https://tokio.rs) ("Tokyo").
- _Tokio is an event-driven, non-blocking I/O platform_:
  - Uses thread pools (green threads and native threads), and processes.
  - Provides fully async versions of standard (synchronous) libraries.
- Async rust code is similar to threading model, but naturally
  scaleable and easier!
- Best for I/O bound applications (CPU bound applications don't benefit from speedups).
  - CPU bound apps can use [rayon](https://docs.rs/rayon/latest/rayon).
  - See also [mio](https://github.com/tokio-rs/mio) for low-level non-blocking
    I/O (`epoll(7)` _et al_).

## async caveats

- Since async functions are scheduled by a runtime, you don't know _where_ or
  _how_ or _when_ they will run!
- May not be a problem, but depends on use-case!

## Add tokio to your project

- Adding the tokio runtime to your project is easy!
- Just run `cargo add`:
  ```bash
  $ cargo add tokio
  ```
- But, to make your life easier, you may want to enable all features
  initially:

  ```bash
  $ cargo add tokio --features full
  ```

  > **Note:**
  >
  > `full` is a tokio-specific "feature" meaning "enable all features"!

## Async: Create an async runtime

```rust
fn main() {
    // XXX: Requires the 'rt' tokio feature!
    // See: https://docs.rs/tokio/latest/tokio/runtime/index.html
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let result = ...; // See below for details! ;)

    if let Err(e) = result {
        eprintln!("ERROR: {e:#?}");
        exit(1);
    }
}

async fn real_main() -> Result<()> {
    // Do stuff... asynchronously! ;)

    Ok(())
}
```

> **Notes:**
>
> - You _can_ use the magic `#[tokio::main]` annotation on
>   `async fn main()` which will generate a runtime for you and run
>   the code you specify in `main()` using that runtime.
>
> - But beware! The default runtime is **not** multi-threaded, so
>   either read the docs for that annotation carefully, or create the
>   runtime manually.

## Async: Create an async function

- Just prefix with `async`:

  ```rust
  async fn foo() -> Result<()> {
    Ok(())
  }
  ```

- The `async` **must** come **immediately before** the
  `fn` keyword:

  ```rust
  pub async fn foo() -> Result<()> {
    Ok(())
  }
  ```

## Async: Call an async function (1)

- First, define an async function:
  ```rust
  async fn foo() -> Result<()> {
      Ok(())
  }
  ```

- Now, what is happening here?
  ```rust
  let x = foo();
  let y = foo();
  ```

- Answer: Not much! The functions have not started running yet!!

## Async: Call and wait for an async function (2)

```rust
async fn foo() -> Result<()> {
    Ok(())
}

let x = foo();
x.await; // Run and wait for the async function to finish

// Same thing one a single line
let y = foo().await;
```

## async and the future!

**Note:** `.await` returns a `Result`.

- Assigning to an async function results in a `Future`.
- A rust [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html)
  is the standard trait (API/interface) for async programming (deferring work).
- `async` keyword generates code to make the function return a `Future`.

```rust
async fn foo() -> Result<()> {
    Ok(())
}

// x implements "Future<Output = Result<(), Error>>"
// but you can't specify this here - it's not a type!
let x = foo();

// result is a Result<()>
let result = x.await; // Schedule, run and wait for the async function to finish
```

## Async: Get result of an async function

Await the function:

```rust
async fn foo() -> Result<()> {
    Ok(())
}

let future = foo(); // The function isn't running yet!

// Note the '?' to return an Err value if the runtime failed to wait
// for the async function.
let result = future.await?;

// At this point, the async function has finished executing
// successfully, so we can check it's return value.
match result {
  Ok(()) => (),
  Err(e) => panic("foo() failed with error: {e:#?}"),
};
```

## Review: synchronous rust template

File [`crates/test-sync-template/src/main.rs`](../crates/test-sync-template/src/main.rs):

```rust
use anyhow::{Result, anyhow};
use std::env;
use std::process::exit;

fn test(value: &str) -> Result<()> {
    println!("INFO: value: {value:?}");

    Ok(())
}

fn real_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let value = args
        .get(1)
        .ok_or("specify string")
        .map_err(|e| anyhow!(e))?;

    test(value)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_name = args
        .first()
        .ok_or("failed to get program name")
        .map_err(|e| anyhow!(e))?;

    let result = real_main();

    if let Err(e) = result {
        eprintln!("ERROR: {program_name}: {e:#?}");
        exit(1);
    }

    result
}
```

## Review: asynchronous rust template

File [`crates/test-tokio-async-template/src/main.rs`](../crates/test-tokio-async-template/src/main.rs):

```rust
use anyhow::{Result, anyhow};
use std::env;
use std::process::exit;

async fn test(value: &str) -> Result<()> {
    println!("INFO: value: {value:?}");

    Ok(())
}

async fn real_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let value = args
        .get(1)
        .ok_or("specify string")
        .map_err(|e| anyhow!(e))?;

    test(value).await
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_name = args
        .first()
        .ok_or("failed to get program name")
        .map_err(|e| anyhow!(e))?;

    // Create a multi-threaded async runtime with all features enabled.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    // Block on allows a synchronous function to run an async function
    // and wait for it to finish.
    let result = rt.block_on(real_main());

    if let Err(e) = result {
        eprintln!("ERROR: {program_name}: {e:#?}");
        exit(1);
    }

    result
}
```

## Compare sync and async templates

```bash
$ diff -u crates/*/src/main.rs
```

```diff
--- crates/test-sync-template/src/main.rs
+++ crates/test-tokio-async-template/src/main.rs
@@ -2,13 +2,13 @@
 use std::env;
 use std::process::exit;

-fn test(value: &str) -> Result<()> {
+async fn test(value: &str) -> Result<()> {
     println!("INFO: value: {value:?}");

     Ok(())
 }

-fn real_main() -> Result<()> {
+async fn real_main() -> Result<()> {
     let args: Vec<String> = env::args().collect();

     let value = args
@@ -16,7 +16,7 @@
         .ok_or("specify string")
         .map_err(|e| anyhow!(e))?;

-    test(value)
+    test(value).await
 }

 fn main() -> Result<()> {
@@ -27,7 +27,14 @@
         .ok_or("failed to get program name")
         .map_err(|e| anyhow!(e))?;

-    let result = real_main();
+    // Create a multi-threaded async runtime with all features enabled.
+    let rt = tokio::runtime::Builder::new_multi_thread()
+        .enable_all()
+        .build()?;
+
+    // Block on allows a synchronous function to run an async function
+    // and wait for it to finish.
+    let result = rt.block_on(real_main());

     if let Err(e) = result {
         eprintln!("ERROR: {program_name}: {e:#?}");
```

## Async: Call and wait for an async function from a synchronous one

- Create a runtime and call the `block_on()` **method**:

  ```rust
  async fn foo() {
    println!("I'm an async function");
  }

  let rt = tokio::runtime::Builder::new_multi_thread()
      .enable_all()
      .build()?;

  let result = rt.block_on( foo() );
  ```

- We can now see the missing line from the runtime creation slide:

  ```rust
  fn main() {
      let rt = tokio::runtime::Builder::new_multi_thread()
          .enable_all()
          .build()?;
  
      // XXX: Here it is: Call the async function "async_main()",
      // XXX: and wait for it to finish.
      let result = rt.block_on(real_main());
  
      if let Err(e) = result {
          eprintln!("ERROR: {e:#?}");
          exit(1);
      }
  }
  
  async fn real_main() -> Result<()> {
      // Do stuff... asynchronously! ;)
  
      Ok(())
  }
  ```

## Async: Call and wait for an async function from a synchronous one (2)

- `cargo add futures` and call the `block_on()` **function**:

  ```rust
  use futures::executor::block_on;

  async fn foo() {
    println!("I'm an async function");
  }

  fn bar() {
      block_on( foo() );
  }
  ```

## Async: Create a new task from an async function

To create a new async task, use `tokio::task::spawn()`

```rust
use tokio::task;

async fn foo() {
    let task_1 = task::spawn(async_function());

    // XXX: Note the async closure.
    let task_2 = task::spawn(async || {
        // Do something ...
    });

    // XXX: Note the async move block.
    let task_3 = task::spawn(async move {
        // Do something ...
    });

    // ...
}
```

> **Note:**
>
> Currently, `async move` **closures** are an unstable feature,
> meaning you can only use them with a "nightly" build of rust.

## Async: Call a synchronous function from an async one

If you need to call a synchronous function from an async function,
create a new task using `spawn_blocking()`:

```rust
use tokio::task;

task::spawn_blocking(|| slow_sync_function());
```

## Async: Call a async function from another one

```rust
async fn foo() -> Result<()> {
    Ok(())
}

async fn bar() -> Result<()> {
    foo().await
}
```

- Question: Why do we need to wait for `foo()` in another async function (`bar()`)?

- Answer: Because if we don't `foo()` may not have finished by the
  time `bar()` returns.

## Async: Wait for first async function to finish

- The `tokio::select!()` macro waits for the first async task to finish.
- It will automatically cancel all remaining tasks waited on.
- Example:

  ```rust
  use tokio::select;
  
  async fn waiter() -> Result<()> {
      // Create some async tasks
      let task1 = tokio::spawn(...);
      let task2 = tokio::spawn(...);
      let task3 = tokio::spawn(...);
  
      // Wait for the first one to finish and kill the rest.
      select! {
          result1 = task1 => { eprintln!("task 1 finished first: result: {result1:?}"); },
          result2 = task2 => { eprintln!("task 2 finished first: result: {result2:?}"); },
          result3 = task3 => { eprintln!("task 3 finished first: result: {result3:?}"); },
      }
  }
  ```

## Async: Wait for all async functions to finish

```rust
use tokio::join;

async fn waiter() -> Result<()> {
    let task1 = tokio::spawn(...);
    let task2 = tokio::spawn(...);
    let task3 = tokio::spawn(...);

    join!(task1, task2, task3);
}
```
## Async: Future

- "Calling" an `async` function with parentheses (round brackets)
  returns a `Future`.
- A `Future` is a _trait_, **NOT** a type!
- Hence, you cannot set the type of a variable to `Future`.
- The `Future` trait defines a single method (`poll()`) to check
  if the value the `Future` represents can be obtained yet.

## Async: Blocks

- sync block of code:

  ```rust
  {
      println!("I am a sync block of code");

      // ...

  }
  ```

- async block of code:

  ```rust
  async {
      println!("I am an async block of code");

      // ...

  }.await
  ```

## Async: Cancellation

Difficult. You need to make the tasks select on a channel.

Here's how I made the agent handle shutdown cleanly:

```rust
use anyhow::{anyhow, Context, Result};
use tokio::sync::watch::channel;

// Create a channel
let (shutdown_tx, shutdown_rx) = channel(true);

// Create the agent logger async task, passing it the reading end of the channel.
let log_handle = tokio::spawn(create_logger_task(rfd, log_vport, shutdown_rx.clone()));

// Start the sandbox and wait for its ttRPC server to end
start_sandbox(&logger, &config, init_mode, &mut tasks, shutdown_rx.clone()).await?;

// Trigger a controlled shutdown
shutdown_tx
    .send(true)
    .map_err(|e| anyhow!(e).context("failed to request shutdown"))?;

// Wait for all threads to finish
let results = join_all(tasks).await;
```

> **Note:** See function `real_main()` in `src/agent/src/main.rs`.

And here's the check for shutdown that the logger code uses:

```rust
loop {
    tokio::select! {
        _ = shutdown.changed() => {
            eprintln!("INFO: interruptable_io_copier: got shutdown request");
            break;
        },

        result = reader.read(&mut buf) => {
            // Copy data if not shutting down...
        },
    };
}
```

> **Note:** See function `interruptable_io_copier()` in `src/agent/src/util.rs`.

## Async: Timeout

It's easy to run a function for a period of time then stop it:

```rust
let timeout = Duration::from_secs(7);

async fn do_something() -> Result<()> { ... }

let result = tokio::time::timeout(timeout, do_something)
            .await
            .context("timed out")?;
```

## Async: Tests

To make a test `async`:

- Add the `async keyword`.
- Add the `#[tokio::test]` annotation.
- Example:
  ```rust
  #[cfg(test)]
  mod tests {
  
      #[tokio::test]
      async fn test_something() {
          // ...
      }
  }
  ```

## Async: Traits

To create an async trait (aka interface / API), you need to do two
things:

- `cargo add async-trait`
- Add `#[async_trait]` to your trait definition

## Async: Trait example

```rust
#[async_trait]
trait Foo: Send + Sync {
	fn sync_operation_1(&self) -> Result<()>;
	fn sync_operation_2(&self) -> Result<()>;

	async fn async_operation_1(&self) -> Result<()>;
	async fn async_operation_2(&self) -> Result<()>;
	async fn async_operation_3(&self) -> Result<()>;
}
```

## Key terms

| Term | Description |
|-|-|
| Tokio | Crate that provides an async runtime and functions |
| `async` | rust keyword to make a function run in parallel |
| `await` | rust keyword to wait for an async function to finish |
| `Future` | rust standard library trait (`std::future::Future`) that represents a runnable async function |
| task | Unit of work represented by a `Future` (an async function) |

## Gotchas

### Missing output from async functions

If you want to add debug messages to your async functions, use
`eprintln!()`.

This writes to `stderr` (unbuffered!)

If you use `println!()`, this writes to `stdout` which may not get
flushed, leading to confusing (or no!) output.

> **Note:** If you must write to `stdout`, use `std::io::stdout().flush().unwrap()`
> **after each `println!()` call**.

### Code hangs

- Runtime hasn't enabled multi-threads.

- You are running a blocking operation in an async function, which blocks
  the entire runtime.

  > **Warning:**
  >
  > - An example of a blocking operation is `thread::sleep()` !
  > - But note that `tokio::time::sleep()` will **not** block: it's
  >   `async`! ;)

## Mix threads and async

- Can you mix async rust and threads?
- Yes!
  ```rust
  async fn my_async_func() {
      let my_thread = std::thread::spawn(|| -> Result<()> {
          // ...

          Ok(())
      });

      // ...

      my_thread
        .join()
        .map_err(|e| anyhow!("{e:?}"))
        .context("failed to join thread")?
        .context("thread function failed")?;
  }
  ```

## Can main be async?

Yes!

```rust
// Magic attribute that auto-creates an async runtime.
#[tokio::main]
async fn main() {
    // ...
}
```

> **Warning:**
>
> - Not recommended since the attribute creates a runtime with an
>   opinionated set of config settings.
> - Safer to call `tokio::runtime::Builder::...` youself!

## Comparison of sync, threaded and async code

| Programming model | Spawn | Run immediately? | Returns | Wait technique |
|-|-|-|-|-|
| synchronous | (function call) | yes | Specified function return type | n/a |
| thread | [`thread::spawn(address)`](https://doc.rust-lang.org/std/thread/fn.spawn.html) | yes | [`JoinHandle`](https://doc.rust-lang.org/std/thread/fn.spawn.html) type `[1]` | `.join()` method |
| `async` | "n/a" | no | `Future` trait | `.await` keyword _et al_ `[3]` |
| `async` task | [`tokio::spawn()`](https://docs.rs/tokio/latest/tokio/fn.spawn.html) | no | [`JoinHandle`](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html) type `[2]` | `.await` keyword _et al_ `[3]` |

<hr/>

- `[1]`: `std::thread::JoinHandle`.
- `[2]`: `tokio::task::JoinHandle`.
- `[3]`: See next slide!

## Async wait methods

| Async wait call | type | Description |
|-|-|-|
| [`.await`](https://doc.rust-lang.org/std/keyword.await.html) | keyword | wait for a single task |
| [`tokio::join!()`](https://docs.rs/tokio/latest/tokio/macro.join.html) | macro | wait for all tasks |
| [`futures::future::join_all()`](https://docs.rs/futures/latest/futures/future/fn.join_all.html) | function | wait for all tasks |
| [`tokio::select!{}`](https://docs.rs/tokio/latest/tokio/macro.select.html) | macro | wait for first task (and kill others) |
| [`tokio::runtime::block_on()`](https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.block_on) | method | wait for all tasks |
| [`futures::executor::block_on()`](https://docs.rs/futures/latest/futures/executor/fn.block_on.html) | function | wait for all tasks |

## References

- [Tokio](https://tokio.rs)
- [Rust async book](https://rust-lang.github.io/async-book)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [Tokio crate docs](https://docs.rs/crate/tokio/latest)
- [Futures crate docs](https://docs.rs/futures/latest/futures)
- [Tokio main attribute](https://docs.rs/tokio/latest/tokio/attr.main.html)

## Finally...

Now, have a play with the bundled async utility. For example,

```bash
$ cargo run -- 'wait-for-all'
```

Task ideas:

- Try changing a few functions to see what happens.

- Try adding a timeout command to run _n_ tasks for a specified period of time, then:
  - kill them all.
  - wait for the first, and then kill the rest.

## The End

Thanks for not falling asleep! ;)
