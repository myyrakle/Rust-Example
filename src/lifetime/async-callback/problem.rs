// Code that doesn't compile

struct Context<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl Context<'_> {
    fn do_something(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

use std::future::Future;

async fn retry<Fut, F>(try_count: usize, callback: F)
where
    F: Fn(&mut Context) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let mut context = Context::<'static> {
        _phantom: std::marker::PhantomData,
    };

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
    retry(3, |context| async {
        println!("Hello, world!");
        context.do_something();

        Ok(())
    })
    .await;
}
