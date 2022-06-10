use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use crate::{TryRecvError, unbounded};

pub trait WorkContext: 'static + Send + Sync + Clone {}
pub trait WorkData: 'static + Send {}
pub trait WorkResult: 'static + Send {}

enum Work<W: WorkData> {
    Close,
    Work(W)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum WorkersStatus {
    Created,
    Running,
    Stopped
}

pub struct Workers<CTX: WorkContext, W: WorkData, R: WorkResult> {
    status: WorkersStatus,
    context: CTX,
    result_receiver: crossbeam::channel::Receiver<R>,
    result_sender: crossbeam::channel::Sender<R>,
    work_receiver: crossbeam::channel::Receiver<Work<W>>,
    work_sender: crossbeam::channel::Sender<Work<W>>,
    workers: Vec<JoinHandle<()>>
}

impl<CTX: WorkContext, W: WorkData, R: WorkResult> Workers<CTX, W, R> {
    pub fn create(context: CTX) -> Self {
        let (result_sender,result_receiver) = unbounded();
        let (work_sender,work_receiver) = unbounded();
        Self {
            status: WorkersStatus::Created,
            context,
            result_receiver,
            result_sender,
            work_receiver,
            work_sender,
            workers: vec![]
        }
    }

    pub fn spin_up(&mut self, work_function: Box<dyn Fn(W, &CTX) -> R + Send + Sync>) -> Result<(),()> {
        if self.status != WorkersStatus::Created {
            return Err(())
        }

        let thread_count = num_cpus::get();

        let work_function = Arc::new(work_function);

        for _ in 0..thread_count {
            let work_receiver = self.work_receiver.clone();
            let result_sender = self.result_sender.clone();

            let context_clone = self.context.clone();
            let work_function_clone = work_function.clone();

            let thread = thread::spawn(move || {
                let context = context_clone;
                let work_function = work_function_clone;

                let closure = &work_function;
                loop {
                    let work = work_receiver.recv();

                    let work = match work {
                        Err(_) => {
                            return;
                        },
                        Ok(work) => {
                            work
                        }
                    };

                    let work = match work {
                        Work::Close => {
                            return;
                        }
                        Work::Work(work) => {
                            work
                        }
                    };

                    let result = closure(work, &context);

                    let send_result = result_sender.send(result);

                    match send_result {
                        Ok(_) => {}
                        Err(_) => {
                            return;
                        }
                    }
                }
            });

            self.workers.push(thread);
        }

        self.status = WorkersStatus::Running;
        Ok(())
    }

    pub fn shut_down(&mut self) -> Result<(),()> {
        if self.status != WorkersStatus::Running {
            return Err(());
        }

        let thread_count = self.workers.len();

        for _ in 0..thread_count {
            self.work_sender.send(Work::Close)
                .map_err(|_| ())?;
        }

        self.status = WorkersStatus::Stopped;
        return Ok(());
    }

    pub fn add_work(&mut self, work: W) -> Result<(),()>{
        if self.status != WorkersStatus::Running {
            return Err(());
        }

        self.work_sender.send(Work::Work(work))
            .map_err(|_| ())?;

        Ok(())
    }

    pub fn receive_result(&mut self) -> Result<R, ()> {
        self.result_receiver.recv().map_err(|_| ())
    }

    pub fn try_receive_result(&mut self) -> Result<Option<R>, ()> {
        let result = self.result_receiver.try_recv();

        match result {
            Err(err) => {
                match err {
                    TryRecvError::Empty => {
                        Ok(None)
                    }
                    TryRecvError::Disconnected => {
                        Err(())
                    }
                }
            }
            Ok(ok) => {
                Ok(Some(ok))
            }
        }
    }

    pub fn has_work_left(&self) -> bool {
        self.work_sender.is_empty()
    }

    pub fn has_results(&self) -> bool {
        !self.result_receiver.is_empty()
    }
}
