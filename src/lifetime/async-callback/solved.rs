struct Context {}

impl Context {
    fn do_something(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

use std::{future::Future, pin::Pin};

async fn retry<F>(try_count: usize, callback: F)
where
    F: Fn(&mut Context) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>>,
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

#[tokio::main]
async fn main() {
    retry(3, |context: &mut Context| {
        Box::pin(async {
            println!("Hello, world!");
            _ = context.do_something();

            Ok(())
        })
    })
    .await;
}
