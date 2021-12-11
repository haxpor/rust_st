/// Demonstrate the use of question mark (?) to propagate the error handling
/// to the caller. It is similar to `match` but not handle error case.
///
/// Thanks to https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html.

mod handling {

    /// Error cases
    enum MathError {
        DivisionByZero,
        NonPositiveSquareRoot,
        LogarithmByNegativeNumber,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NonPositiveSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        // logarithm of negative or zero are not defined
        if x <= 0.0 {
            Err(MathError::LogarithmByNegativeNumber)
        } else {
            Ok(x.ln())
        }
    }

    fn op_(x: f64, y: f64) -> MathResult {
        // use ? to propagate error (if any) to the caller
        let ratio = div(x, y)?;

        // use ? to propagate error (if any) to the caller
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::DivisionByZero => "division by zero",
                MathError::NonPositiveSquareRoot => "square root of negative number",
                MathError::LogarithmByNegativeNumber => "logarithm of non-positive number"
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

fn main() {
    // this should panic the program
    handling::op(1.0, 10.0);   
}
