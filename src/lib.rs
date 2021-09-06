use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Success,
    Failure,
    Running,
}

pub trait Node {
    fn tick (&mut self) -> Status;
    fn name (&self) -> String { "none".into() }
    fn reset (&mut self) {}
}

pub mod referenced {
    mod selector {
        use crate::{Node, Status};

        pub struct Selector<'a, const N: usize> {
            name: String,
            tasks: [&'a mut dyn Node; N],
        }

        impl<'a, const N: usize> Selector<'a, N> {
            pub fn new(name: String, tasks: [&'a mut dyn Node; N]) -> Selector<'a, N> {
                Selector { name, tasks }
            }
        }

        impl<'a, const N: usize> Node for Selector<'a, N> {
            fn tick(&mut self) -> Status {
                for task in self.tasks.iter_mut() {
                    match task.tick() {
                        Status::Success => return Status::Success,
                        Status::Failure => {}
                        Status::Running => return Status::Running,
                    }
                }
                Status::Failure
            }
            fn name (&self) -> String {
                self.name.clone()
            }
        }
    }

    mod sequence {
        use crate::{Node, Status};

        pub struct Sequence<'a, const N: usize> {
            name: String,
            tasks: [&'a mut dyn Node; N],
        }

        impl<'a, const N: usize> Sequence<'a, N> {
            pub fn new(name: String, tasks: [&'a mut dyn Node; N]) -> Sequence<'a, N> {
                Sequence { name, tasks }
            }
        }

        impl<'a, const N: usize> Node for Sequence<'a, N> {
            fn tick(&mut self) -> Status {
                for task in self.tasks.iter_mut() {
                    match task.tick() {
                        Status::Success => {}
                        Status::Failure => return Status::Failure,
                        Status::Running => return Status::Running,
                    }
                }
                Status::Success
            }
            fn name (&self) -> String {
                self.name.clone()
            }
        }
    }

    pub use selector::Selector;
    pub use sequence::Sequence;
}

pub mod boxed {
    mod selector {
        use crate::{Node, Status};

        pub struct Selector<const N: usize> {
            name: String,
            tasks: [Box<dyn Node>; N],
        }

        impl<const N: usize> Selector<N> {
            pub fn new(name: String, tasks: [Box<dyn Node>; N]) -> Selector<N> {
                Selector { name, tasks }
            }
        }

        impl<const N: usize> Node for Selector<N> {
            fn tick(&mut self) -> Status {
                for task in self.tasks.iter_mut() {
                    match task.tick() {
                        Status::Success => return Status::Success,
                        Status::Failure => {}
                        Status::Running => return Status::Running,
                    }
                }
                Status::Failure
            }
            fn name (&self) -> String {
                self.name.clone()
            }
        }
    }

    mod sequence {
        use crate::{Node, Status};

        pub struct Sequence<const N: usize> {
            name: String,
            tasks: [Box<dyn Node>; N],
        }

        impl<const N: usize> Sequence<N> {
            pub fn new(name: String, tasks: [Box<dyn Node>; N]) -> Sequence<N> {
                Sequence { name, tasks }
            }
        }

        impl<const N: usize> Node for Sequence<N> {
            fn tick(&mut self) -> Status {
                for task in self.tasks.iter_mut() {
                    match task.tick() {
                        Status::Success => {}
                        Status::Failure => return Status::Failure,
                        Status::Running => return Status::Running,
                    }
                }
                Status::Success
            }
            fn name (&self) -> String {
                self.name.clone()
            }
        }
    }

    pub use selector::Selector;
    pub use sequence::Sequence;
}

pub struct AlwaysSuccess;

impl Node for AlwaysSuccess {
    fn tick(&mut self) -> Status {
        Status::Success
    }
}

pub struct AlwaysFailure;

impl Node for AlwaysFailure {
    fn tick(&mut self) -> Status {
        Status::Failure
    }
}

pub struct AlwaysRunning;

impl Node for AlwaysRunning {
    fn tick(&mut self) -> Status {
        Status::Running
    }
}

pub struct Wait {
    duration: Duration,
    start: Option<Instant>,
}

impl Wait {
    pub fn new (duration: Duration) -> Wait {
        Wait {
            duration,
            start: None,
        }
    }
}

impl Node for Wait {
    fn tick(&mut self) -> Status {
        match self.start {
            None => {
                self.start = Some(Instant::now());
                Status::Running
            }
            Some(ref start) => {
                if start.elapsed() >= self.duration {
                    Status::Success
                } else {
                    Status::Running
                }
            }
        }
    }
}

mod decorators {
    use crate::{Node, Status};

    struct Once {
        done: Option<Status>,
        node: Box<dyn Node>
    }

    impl Once {
        fn new (node: Box<dyn Node>) -> Once {
            Once { done: None, node }
        }
    }

    impl Node for Once {
        fn tick(&mut self) -> Status {
            if let Some(status) = self.done {
                status
            } else {
                match self.node.tick() {
                    Status::Running => Status::Running,
                    status => { self.done = Some(status); status }
                }
            }
        }
    }
}

#[cfg(test)]
mod test;