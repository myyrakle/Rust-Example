// since 1.85.0

struct Context {}

impl Context {
    fn do_something(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

async fn retry<F>(try_count: usize, callback: F)
where
    F: AsyncFn(&mut Context) -> anyhow::Result<()>,
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
    retry(3, async |context: &mut Context| {
        println!("Hello, world!");
        context.do_something();

        Ok(())
    })
    .await;
}
