struct Context {}

impl Context {
    fn do_something(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

use std::future::Future;

async fn retry<F>(try_count: usize, mut callback: F)
where
    F: for<'a> AsyncCallback<'a, ()>,
{
    let mut context = Context {};

    let mut attempts = 0;
    loop {
        attempts += 1;
        match callback(&mut context).await {
            Ok(_) => break,
            Err(e) => {
                if attempts >= try_count {
                    eprintln!("Failed after 3 attempts: {}", e);
                    break;
                }
            }
        }
    }
}

trait AsyncCallback<'a, T: 'a>: FnMut(&'a mut Context) -> Self::Fut {
    type Fut: Future<Output = anyhow::Result<T>>;
}

impl<'a, T: 'a, Out: Future<Output = anyhow::Result<T>>, F: Fn(&'a mut Context) -> Out>
    AsyncCallback<'a, T> for F
{
    type Fut = Out;
}

async fn callback_test(context: &mut Context) -> anyhow::Result<()> {
    println!("Hello, world!");
    context.do_something()
}

#[tokio::main]
async fn main() {
    retry(3, callback_test).await;
}
