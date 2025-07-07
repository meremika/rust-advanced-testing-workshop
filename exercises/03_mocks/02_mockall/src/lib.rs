//! Use `mockall` to mock the `Logger` trait in the `square` function.\
//! Use the generated mock in the test.
pub fn square<L>(x: i32, logger: L) -> i32
where
    L: Logger,
{
    let y = x * x;
    logger.log(&format!("{}^2 == {}", x, y));
    y
}

#[mockall::automock]
pub trait Logger {
    fn log(&self, msg: &str);
}

pub struct PrintlnLogger;

impl Logger for PrintlnLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use crate::MockLogger;

    use super::square;

    #[test]
    fn square_works() {
        let mut mock_logger = MockLogger::new();
        mock_logger.expect_log().times(1).returning(|_| {});
        assert_eq!(square(2, mock_logger), 4);
    }
}
