use std::{
    future::Future,
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};

pub struct SetTimeout {
    pub shared_state: Arc<Mutex<SharedState>>,
    pub done: Arc<AtomicBool>,
}

pub struct SharedState {
    pub waker: Option<Waker>,
}

impl SetTimeout {
    pub fn new<F>(delay: usize, callback: F) -> Self
    where
        F: FnOnce() + Send + 'static + Clone,
    {
        let _shared_state = Arc::new(Mutex::new(SharedState { waker: None }));
        let _done = Arc::new(AtomicBool::new(false));
        let done = _done.clone();

        let shared_state = _shared_state.clone();
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(delay as u64));
            callback();
            done.store(true, std::sync::atomic::Ordering::Relaxed);

            if let Some(waker) = shared_state.lock().unwrap().waker.take() {
                // 아빠! 일어나!
                waker.wake();
            }
        });

        SetTimeout {
            done: Arc::new(AtomicBool::new(false)),
            shared_state: _shared_state,
        }
    }
}

impl Future for SetTimeout {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.done.load(std::sync::atomic::Ordering::Acquire) {
            std::task::Poll::Ready(())
        } else {
            {
                let mut shared_state = self.shared_state.lock().unwrap();
                shared_state.waker = Some(context.waker().clone()); // 여기서 주입
            }

            std::task::Poll::Pending
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        let future = SetTimeout::new(1000, || {
            println!("2");
        });
        future.await;
        println!("1"); // 왜 실행안됨??
    });

    // "spawner"를 삭제하여 executor가 더 이상 실행할 작업이 없음을 알고 더 이상 들어오는 작업을 받지 않도록 합니다.
    drop(spawner);

    // 이 executor는 작업 큐가 비어 있을 때까지 실행됩니다. 이렇게하면 "howdy!"를 인쇄하고 일시 중지 한 다음 "done!"을 인쇄합니다.
    executor.run();
}

use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::task::{Context, Waker};

/// executor는 채널에서 Task를 받아 실행하는 프로그램입니다.
pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// Spawner는 새로운 future들을 task channel에 생성합니다.
#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// "Executor"에게 polling되기 위해 스스로를 reschedule할 수 있는 "Future"입니다.
pub struct Task {
    // 진행 중인 futures들.
    // Mutex는 싱글스레드라서 꼭 필요하지는 않습니다. 문법을 회피하기 위해 넣었고, 대신 UnsafeCell을 써도 됩니다.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 스스로를 task queue에 넣는 Handle (다른 작업을 위해 일시중지할때 사용)
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 한 번에 큐에 허용되는 최대 작업 수.
    // sync_channel을 단순하기 위해 만든거고, 상용 코드에서는 하지 않습니다.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 이 task를 다시 task channel로 전송하여 executor에 의해 다시 폴링되도록 'wake'를 구현한다.
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // future를 꺼내와서 아직 완료되지 않은 경우, 완료를 재촉하기 위해 poll을 날립니다.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // task 자체를 통해 LocalWaker를 만듭니다.
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>`는 `Pin<Box<dyn Future<Output = T> + Send + 'static>>`에 대한 alias
                if let std::task::Poll::Pending = future.as_mut().poll(context) {
                    // future 완료되지 않았다면 다시 task에 돌려둡니다.
                    *future_slot = Some(future);
                }
            }
        }
    }
}
