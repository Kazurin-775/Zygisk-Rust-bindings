/// An opaque type that represents an error originated from Zygisk.
///
/// Since Zygisk does not make use of `errno`, it is not really possible for us
/// to know the actual cause without using `logcat`. This type only acts as a
/// reminder to inform the user that `logcat` MAY contain useful information
/// for diagnostics.
#[derive(Clone)]
pub struct ZygiskError;

impl std::fmt::Debug for ZygiskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZygiskError")
            .field("_note", &"see logcat for details")
            .finish()
    }
}

impl std::fmt::Display for ZygiskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ERROR_MSG: &str = "an error occurred within Zygisk (see logcat for details)";

        f.write_str(ERROR_MSG)
    }
}

impl std::error::Error for ZygiskError {}

#[test]
fn test_debug_fmt() {
    assert_eq!(
        format!("{:?}", ZygiskError),
        "ZygiskError { _note: \"see logcat for details\" }",
    );
}
