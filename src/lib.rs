use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Success,
    Failure,
    Running,
}

pub trait Node {
    fn tick (&mut self, depth: usize, debug: &mut Option<Vec<(usize, String)>>) -> Status;
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
            fn tick(&mut self, depth: usize, debug: &mut Option<Vec<(usize, String)>>) -> Status {
                for task in self.tasks.iter_mut() {
                    match task.tick(depth, debug) {
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
            fn tick(&mut self, depth: usize, debug: &mut Option<Vec<(usize, String)>>) -> Status {
                for task in self.tasks.iter_mut() {
                    match task.tick(depth, debug) {
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
            fn tick(&mut self, depth: usize, debug: &mut Option<Vec<(usize, String)>>) -> Status {
                if let Some(ref mut debug) = debug {
                    debug.push((depth, self.name()));
                }
                for task in self.tasks.iter_mut() {
                    match task.tick(depth + 1, debug) {
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
            fn tick(&mut self, depth: usize, debug: &mut Option<Vec<(usize, String)>>) -> Status {
                if let Some(ref mut debug) = debug {
                    debug.push((depth, self.name()));
                }
                for task in self.tasks.iter_mut() {
                    match task.tick(depth + 1, debug) {
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
    fn tick(&mut self, depth: usize, debug: &mut Option<Vec<(usize, String)>>) -> Status {
        if let Some(ref mut debug) = debug {
            debug.push((depth, self.name()));
        }
        Status::Success
    }
    fn name(&self) -> String {
        "always-success".into()
    }
}

pub struct AlwaysFailure;

impl Node for AlwaysFailure {
    fn tick(&mut self, depth: usize, debug: &mut Option<Vec<(usize, String)>>) -> Status {
        if let Some(ref mut debug) = debug {
            debug.push((depth, self.name()));
        }
        Status::Failure
    }
    fn name(&self) -> String {
        "always-failure".into()
    }
}

pub struct AlwaysRunning;

impl Node for AlwaysRunning {
    fn tick(&mut self, depth: usize, debug: &mut Option<Vec<(usize, String)>>) -> Status {
        if let Some(ref mut debug) = debug {
            debug.push((depth, self.name()));
        }
        Status::Running
    }
    fn name(&self) -> String {
        "always-running".into()
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
    fn tick(&mut self, depth: usize, debug: &mut Option<Vec<(usize, String)>>) -> Status {
        if let Some(ref mut debug) = debug {
            debug.push((depth, self.name()));
        }
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
    fn name (&self) -> String {
        let duration = self.duration.as_millis();
        if let Some(start) = self.start {
            let elapsed = start.elapsed().as_millis();
            format!("wait {}", if duration > elapsed { duration - elapsed } else { 0 })
        } else {
            format!("wait {}", duration)
        }
    }
}

mod decorators {
    use crate::{Node, Status};

    pub struct Once {
        done: Option<Status>,
        node: Box<dyn Node>
    }

    impl Once {
        pub fn new (node: Box<dyn Node>) -> Once {
            Once { done: None, node }
        }
    }

    impl Node for Once {
        fn tick(&mut self, depth: usize, debug: &mut Option<Vec<(usize, String)>>) -> Status {
            if let Some(ref mut debug) = debug {
                debug.push((depth, self.name()));
            }
            if let Some(status) = self.done {
                status
            } else {
                match self.node.tick(depth + 1, debug) {
                    Status::Running => Status::Running,
                    status => { self.done = Some(status); status }
                }
            }
        }
        fn name(&self) -> String {
            if let Some(status) = self.done {
                format!("once cached {:?}", status)
            } else {
                "once".into()
            }
        }
    }
}

pub use decorators::Once;

#[cfg(test)]
mod test;