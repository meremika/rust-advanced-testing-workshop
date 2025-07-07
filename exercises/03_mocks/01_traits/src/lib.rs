//! Refactor the `square` function to ask for a type that implements the `Logger` trait rather than the concrete
//! `PrintlnLogger` type.\
//! Then pass a `TestLogger` to `square` in the test. `TestLogger` should implement `Logger` and do nothing
//! when `log` is called.

pub trait Logger {
    fn log(&self, msg: &str);
}

pub struct PrintlnLogger;


pub fn square<L: Logger>(x: i32, logger: L) -> i32 {
    let y = x * x;
    logger.log(&format!("{x}^2 == {y}"));
    y
}


impl Logger for PrintlnLogger {
    fn log(&self, msg: &str) {
        println!("{msg}");
    }
}

#[cfg(test)]
mod tests {
    use super::{square, Logger};
    use googletest::assert_that;
    use googletest::matchers::eq;

    struct TestLogger;

    impl Logger for TestLogger {
        fn log(&self, _msg: &str) {}
    }

    #[test]
    fn square_works() {
        assert_that!(square(2, TestLogger), eq(4));
    }
}
