mod referenced {
    use crate::{Node, Status, referenced::{Sequence, Selector}};

    struct Success(usize);

    impl Success {
        fn new(number: usize) -> Success {
            Success(number)
        }
    }

    impl Node for Success {
        fn tick(&mut self, _depth: usize, _debug: &mut Option<Vec<(usize, String)>>) -> Status {
            println!("success({})", self.0);
            Status::Success
        }
    }

    struct Fail(usize);

    impl Fail {
        fn new(number: usize) -> Fail {
            Fail(number)
        }
    }

    impl Node for Fail {
        fn tick(&mut self, _depth: usize, _debug: &mut Option<Vec<(usize, String)>>) -> Status {
            println!("fail({})", self.0);
            Status::Failure
        }
    }

    #[test]
    fn selector_success() {
        let mut s1 = Success::new(1);
        let mut s2 = Success::new(2);
        let mut root = Selector::new("root".into(), [&mut s1, &mut s2]);
        let status = root.tick(0, &mut None);
        println!("selector {:?}", status);
    }

    #[test]
    fn sequence_success() {
        let mut s1 = Success::new(1);
        let mut s2 = Success::new(2);
        let mut root = Sequence::new("root".into(), [&mut s1, &mut s2]);
        let status = root.tick(0, &mut None);
        println!("sequence {:?}", status);
    }

    #[test]
    fn selector_fail() {
        let mut s1 = Fail::new(1);
        let mut s2 = Fail::new(2);
        let mut root = Selector::new("root".into(), [&mut s1, &mut s2]);
        let status = root.tick(0, &mut None);
        println!("selector {:?}", status);
    }

    #[test]
    fn sequence_fail() {
        let mut s1 = Fail::new(1);
        let mut s2 = Fail::new(2);
        let mut root = Sequence::new("root".into(), [&mut s1, &mut s2]);
        let status = root.tick(0, &mut None);
        println!("sequence {:?}", status);
    }

    #[test]
    fn composite() {
        let mut s1 = Success::new(1);
        let mut s2 = Fail::new(2);
        let mut root = Sequence::new("root".into(), [&mut s1, &mut s2]);
        let mut s3 = Fail::new(3);
        let mut root2 = Sequence::new("root".into(), [&mut root, &mut s3]);
        let status = root2.tick(0, &mut None);
        println!("root2 {:?}", status);
    }
}

mod boxed {
    use crate::{Node, Status, boxed::{Sequence, Selector}};

    struct Success(usize);

    impl Success {
        fn new(number: usize) -> Success {
            Success(number)
        }
    }

    impl Node for Success {
        fn tick(&mut self, _depth: usize, _debug: &mut Option<Vec<(usize, String)>>) -> Status {
            println!("success({})", self.0);
            Status::Success
        }
    }

    struct Fail(usize);

    impl Fail {
        fn new(number: usize) -> Fail {
            Fail(number)
        }
    }

    impl Node for Fail {
        fn tick(&mut self, _depth: usize, _debug: &mut Option<Vec<(usize, String)>>) -> Status {
            println!("fail({})", self.0);
            Status::Failure
        }
    }

    #[test]
    fn selector_success() {
        let s1 = Success::new(1);
        let s2 = Success::new(2);
        let mut root = Selector::new("root".into(), [Box::new(s1), Box::new(s2)]);
        let status = root.tick(0, &mut None);
        println!("selector {:?}", status);
    }

    #[test]
    fn sequence_success() {
        let s1 = Success::new(1);
        let s2 = Success::new(2);
        let mut root = Sequence::new("root".into(), [Box::new(s1), Box::new(s2)]);
        let status = root.tick(0, &mut None);
        println!("sequence {:?}", status);
    }

    #[test]
    fn selector_fail() {
        let s1 = Fail::new(1);
        let s2 = Fail::new(2);
        let mut root = Selector::new("root".into(), [Box::new(s1), Box::new(s2)]);
        let status = root.tick(0, &mut None);
        println!("selector {:?}", status);
    }

    #[test]
    fn sequence_fail() {
        let s1 = Fail::new(1);
        let s2 = Fail::new(2);
        let mut root = Sequence::new("root".into(), [Box::new(s1), Box::new(s2)]);
        let status = root.tick(0, &mut None);
        println!("sequence {:?}", status);
    }

    #[test]
    fn composite() {
        let nested = || {
            let s1 = Success::new(1);
            let s2 = Fail::new(2);
            let nested = Sequence::new("nested".into(), [Box::new(s1), Box::new(s2)]);
            nested
        };
        let nested = nested();
        let s3 = Fail::new(3);
        let mut root = Sequence::new("root".into(), [Box::new(nested), Box::new(s3)]);
        let status = root.tick(0, &mut None);
        println!("root2 {:?}", status);
    }
}