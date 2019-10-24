#![allow(dead_code)]
use std::panic;

type TestFunction = fn() -> ();

pub struct Tester {
    before: Option<TestFunction>,
    after: Option<TestFunction>,
}

impl Tester {
    pub fn new() -> Self {
        Tester {
            before: None,
            after: None,
        }
    }
    pub fn set_before(mut self, before: TestFunction) -> Self {
        self.before = Some(before);
        self
    }

    pub fn set_after(mut self, after: TestFunction) -> Self {
        self.after = Some(after);
        self
    }

    pub fn run<F>(&self, test: F)
    where
        F: FnOnce() -> () + panic::UnwindSafe,
    {
        if let Some(before) = self.before {
            before();
        }
        let result = panic::catch_unwind(test);
        if let Some(after) = self.after {
            after();
        }
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tester() {
        let t = Tester::new().set_before(|| assert!(true)).set_after(|| {});
        t.run(|| {
            assert!(1 + 1 == 2);
        });
        t.run(|| {
            assert!(true);
        })
    }
}
