use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyTypeError};
use pyo3::prelude::*;

use pgpq::error::ErrorKind;

create_exception!(exceptions, FieldTooLarge, PyException);
create_exception!(exceptions, EncodeError, PyException);
create_exception!(exceptions, EncodingNotSupported, PyException);
create_exception!(exceptions, EncoderMissing, PyException);
create_exception!(exceptions, UnknownFields, PyException);

/// Newtype pattern wrapper for ErrorKind.
pub struct PgPqErrorKind(pub ErrorKind);

impl std::convert::From<PgPqErrorKind> for PyErr {
    fn from(err: PgPqErrorKind) -> PyErr {
        match err.0 {
            ErrorKind::ColumnTypeMismatch { .. }
            | ErrorKind::TypeNotSupported { .. }
            | ErrorKind::FieldTypeNotSupported { .. } => {
                PyErr::new::<PyTypeError, _>(format!("{}", err.0))
            }
            ErrorKind::FieldTooLarge { .. } => PyErr::new::<FieldTooLarge, _>(format!("{}", err.0)),
            ErrorKind::Encode { .. } => PyErr::new::<EncodeError, _>(format!("{}", err.0)),
            ErrorKind::EncoderMissing { .. } => {
                PyErr::new::<EncoderMissing, _>(format!("{}", err.0))
            }
            ErrorKind::EncodingNotSupported { .. } => {
                PyErr::new::<EncodingNotSupported, _>(format!("{}", err.0))
            }
            ErrorKind::UnknownFields { .. } => PyErr::new::<UnknownFields, _>(format!("{}", err.0)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pyo3::types::IntoPyDict;

    #[test]
    fn test_exceptions_available_in_exceptions_module() {
        Python::with_gil(|py| {
            let ctx = [("FieldTooLarge", py.get_type::<FieldTooLarge>())].into_py_dict(py);
        });
    }
}
