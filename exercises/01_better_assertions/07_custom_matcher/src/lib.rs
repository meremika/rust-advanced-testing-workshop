//! Write a custom `is_redirect` matcher that checks if a `StatusCode` is a redirect.
use googletest::{matcher::Matcher, prelude::predicate};
use http::StatusCode;

pub fn is_redirect() -> impl Matcher<StatusCode> {
    predicate(|s: StatusCode| s.is_redirection()).with_description(
        "is a redirection status code",
        "isn't a redirection status code",
    )
}

#[cfg(test)]
mod tests {
    use crate::is_redirect;
    use googletest::assert_that;
    use http::StatusCode;

    #[test]
    fn success() {
        assert_that!(StatusCode::MOVED_PERMANENTLY, is_redirect());
    }

    #[test]
    fn failure() {
        assert_that!(StatusCode::OK, is_redirect());
    }
}
