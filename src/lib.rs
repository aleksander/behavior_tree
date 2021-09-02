#[derive(Debug)]
pub enum Status {
    Success,
    Failure,
    Running,
}

pub trait Node {
    fn tick (&mut self) -> Status;
}

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

#[cfg(test)]
mod tests {
    use crate::{Sequence, Node, Status, Selector};

    struct Success(usize);

    impl Success {
        fn new (number: usize) -> Success {
            Success(number)
        }
    }

    impl Node for Success {
        fn tick (&mut self) -> Status {
            println!("success({})", self.0);
            Status::Success
        }
    }

    struct Fail(usize);

    impl Fail {
        fn new (number: usize) -> Fail {
            Fail(number)
        }
    }

    impl Node for Fail {
        fn tick (&mut self) -> Status {
            println!("fail({})", self.0);
            Status::Failure
        }
    }

    #[test]
    fn selector_success () {
        let mut s1 = Success::new(1);
        let mut s2 = Success::new(2);
        let mut root = Selector::new([&mut s1, &mut s2]);
        let status = root.tick();
        println!("selector {:?}", status);
    }

    #[test]
    fn sequence_success () {
        let mut s1 = Success::new(1);
        let mut s2 = Success::new(2);
        let mut root = Sequence::new([&mut s1, &mut s2]);
        let status = root.tick();
        println!("sequence {:?}", status);
    }

    #[test]
    fn selector_fail () {
        let mut s1 = Fail::new(1);
        let mut s2 = Fail::new(2);
        let mut root = Selector::new([&mut s1, &mut s2]);
        let status = root.tick();
        println!("selector {:?}", status);
    }

    #[test]
    fn sequence_fail () {
        let mut s1 = Fail::new(1);
        let mut s2 = Fail::new(2);
        let mut root = Sequence::new([&mut s1, &mut s2]);
        let status = root.tick();
        println!("sequence {:?}", status);
    }

    #[test]
    fn composite () {
        let mut s1 = Fail::new(1);
        let mut s2 = Fail::new(2);
        let mut root = Sequence::new([&mut s1, &mut s2]);
        let mut s3 = Fail::new(3);
        let mut root2 = Sequence::new([&mut root, &mut s3]);
        let status = root2.tick();
        println!("root2 {:?}", status);
    }
}
