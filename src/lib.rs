#[derive(Debug)]
pub enum Status {
    Success,
    Failure,
    Running,
}

pub trait Node {
    fn tick (&mut self) -> Status;
}

pub mod referenced {
    mod selector {
        use crate::{Node, Status};

        pub struct Selector<'a, const N: usize> {
            tasks: [&'a mut dyn Node; N],
        }

        impl<'a, const N: usize> Selector<'a, N> {
            pub fn new(tasks: [&'a mut dyn Node; N]) -> Selector<'a, N> {
                Selector {
                    tasks
                }
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
        }
    }

    mod sequence {
        use crate::{Node, Status};

        pub struct Sequence<'a, const N: usize> {
            tasks: [&'a mut dyn Node; N],
        }

        impl<'a, const N: usize> Sequence<'a, N> {
            pub fn new(tasks: [&'a mut dyn Node; N]) -> Sequence<'a, N> {
                Sequence {
                    tasks
                }
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
        }
    }

    pub use selector::Selector;
    pub use sequence::Sequence;
}

pub mod boxed {
    mod selector {
        use crate::{Node, Status};

        pub struct Selector<const N: usize> {
            tasks: [Box<dyn Node>; N],
        }

        impl<const N: usize> Selector<N> {
            pub fn new(tasks: [Box<dyn Node>; N]) -> Selector<N> {
                Selector {
                    tasks
                }
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
        }
    }

    mod sequence {
        use crate::{Node, Status};

        pub struct Sequence<const N: usize> {
            tasks: [Box<dyn Node>; N],
        }

        impl<const N: usize> Sequence<N> {
            pub fn new(tasks: [Box<dyn Node>; N]) -> Sequence<N> {
                Sequence {
                    tasks
                }
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

#[cfg(test)]
mod test;