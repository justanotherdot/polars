// Credits to https://github.com/omerbenamram/pyo3-file
use parquet::{
    errors::ParquetError,
    file::reader::{Length, TryClone},
};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use std::borrow::Borrow;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Clone)]
pub struct PyFileLikeObject {
    inner: PyObject,
}

/// Wraps a `PyObject`, and implements read, seek, and write for it.
impl PyFileLikeObject {
    /// Creates an instance of a `PyFileLikeObject` from a `PyObject`.
    /// To assert the object has the required methods methods,
    /// instantiate it with `PyFileLikeObject::require`
    pub fn new(object: PyObject) -> Self {
        PyFileLikeObject { inner: object }
    }

    /// Same as `PyFileLikeObject::new`, but validates that the underlying
    /// python object has a `read`, `write`, and `seek` methods in respect to parameters.
    /// Will return a `TypeError` if object does not have `read`, `seek`, and `write` methods.
    pub fn with_requirements(
        object: PyObject,
        read: bool,
        write: bool,
        seek: bool,
    ) -> PyResult<Self> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        if read {
            if let Err(_) = object.getattr(py, "read") {
                return Err(PyErr::new::<PyTypeError, _>(
                    "Object does not have a .read() method.",
                ));
            }
        }

        if seek {
            if let Err(_) = object.getattr(py, "seek") {
                return Err(PyErr::new::<PyTypeError, _>(
                    "Object does not have a .seek() method.",
                ));
            }
        }

        if write {
            if let Err(_) = object.getattr(py, "write") {
                return Err(PyErr::new::<PyTypeError, _>(
                    "Object does not have a .write() method.",
                ));
            }
        }

        Ok(PyFileLikeObject::new(object))
    }
}

/// Extracts a string repr from, and returns an IO error to send back to rust.
fn pyerr_to_io_err(e: PyErr) -> io::Error {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let e_as_object: PyObject = e.into_py(py);

    match e_as_object.call_method(py, "__str__", (), None) {
        Ok(repr) => match repr.extract::<String>(py) {
            Ok(s) => io::Error::new(io::ErrorKind::Other, s),
            Err(_e) => io::Error::new(io::ErrorKind::Other, "An unknown error has occurred"),
        },
        Err(_) => io::Error::new(io::ErrorKind::Other, "Err doesn't have __str__"),
    }
}

impl Read for PyFileLikeObject {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, io::Error> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let bytes = self
            .inner
            .call_method(py, "read", (buf.len(),), None)
            .map_err(pyerr_to_io_err)?;

        let bytes: &PyBytes = bytes
            .cast_as(py)
            .expect("Expecting to be able to downcast into bytes from read result.");

        &buf.write(bytes.as_bytes())?;

        Ok(bytes.len().map_err(pyerr_to_io_err)?)
    }
}

impl Write for PyFileLikeObject {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let pybytes = PyBytes::new(py, buf);

        let number_bytes_written = self
            .inner
            .call_method(py, "write", (pybytes,), None)
            .map_err(pyerr_to_io_err)?;

        Ok(number_bytes_written.extract(py).map_err(pyerr_to_io_err)?)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        self.inner
            .call_method(py, "flush", (), None)
            .map_err(pyerr_to_io_err)?;

        Ok(())
    }
}

impl Seek for PyFileLikeObject {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, io::Error> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let (whence, offset) = match pos {
            SeekFrom::Start(i) => (0, i as i64),
            SeekFrom::Current(i) => (1, i as i64),
            SeekFrom::End(i) => (2, i as i64),
        };

        let new_position = self
            .inner
            .call_method(py, "seek", (offset, whence), None)
            .map_err(pyerr_to_io_err)?;

        Ok(new_position.extract(py).map_err(pyerr_to_io_err)?)
    }
}

pub trait FileLike: Read + Write + Seek {}

// Needed for arrow parquet
impl Length for PyFileLikeObject {
    fn len(&self) -> u64 {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let size = self
            .inner
            .call_method0(py, "__sizeof__")
            .expect("Could not read size of buffer");
        size.extract(py)
            .expect("did not get an int as result of __sizeof__")
    }
}

impl TryClone for PyFileLikeObject {
    fn try_clone(&self) -> std::result::Result<Self, ParquetError> {
        Ok(self.clone())
    }
}

impl FileLike for File {}
impl FileLike for PyFileLikeObject {}

pub enum EitherRustPythonFile {
    Py(PyFileLikeObject),
    Rust(File),
}

pub fn get_either_file(py_f: PyObject, truncate: bool) -> PyResult<EitherRustPythonFile> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    if let Ok(pstring) = py_f.cast_as::<PyString>(py) {
        let rstring = pstring.to_string();
        let str_slice: &str = rstring.borrow();
        let f = if truncate {
            File::create(str_slice)?
        } else {
            File::open(str_slice)?
        };
        Ok(EitherRustPythonFile::Rust(f))
    } else {
        let f = PyFileLikeObject::with_requirements(py_f, true, true, true)?;
        Ok(EitherRustPythonFile::Py(f))
    }
}

pub fn get_file_like(f: PyObject, truncate: bool) -> PyResult<Box<dyn FileLike>> {
    use EitherRustPythonFile::*;
    match get_either_file(f, truncate)? {
        Py(f) => Ok(Box::new(f)),
        Rust(f) => Ok(Box::new(f)),
    }
}
