use std::path::PathBuf;
//use std::fmt::Display;
// use std::fmt;
// use std;
// use futures::future::ExecuteError;

#[derive(Fail, Debug)]
pub enum MediaError {
    #[fail(display = "{}", _0)]
    Invalid(String),
    #[fail(display = "Execute error")]
    ExecuteError,
    #[fail(display = "Not a directory: {:?}", _0)]
    NotADirectory(PathBuf),
    #[fail(display = "Not a file: {:?}", _0)]
    NotAFile(String),
    #[fail(display = "Not found: {}", _0)]
    NotFound(String),
    #[fail(display = "{}", _0)]
    Other(&'static str),
    #[fail(display = "{}", _0)]
    Unimplemented(&'static str),
    #[fail(display = "{}", _0)]
    Message(String),
    #[fail(display = "{}", _0)]
    Unsupported(&'static str),
}

//impl ser::Error for MediaError {
//    fn custom<T: Display>(msg: T) -> Self {
//        MediaError::Message(msg.to_string())
//    }
//}

//error_chain!{
//    errors {
//        ExecuteError
//    }
//
//    foreign_links {
//        Hyper(::hyper::Error);
//        Io(::std::io::Error);
//        Json(::serde_json::Error);
//        KXml(::xml::Error);
//        Nix(::nix::Error);
//        Utf8Error(::std::str::Utf8Error);
//        Xml(::serde_xml_rs::Error);
//    }
//}
//
//impl From<MediaError> for Error {
//    fn from(_: MediaError) -> Self {
//        ErrorKind::ExecuteError.into()
//    }
//}
//
//impl<T> Into<futures::sync::mpsc::SendError<T>> for Error {
//    fn into(self) -> futures::sync::mpsc::SendError<T> {
//        panic!("Can't convert following into futures::sync::mpsc::SendError: {:?}", self)
//    }
//}
//
//impl<T> From<futures::sync::mpsc::SendError<T>> for Error {
//    fn from(err: futures::sync::mpsc::SendError<T>) -> Self {
//        MediaError::Other(format!("SendError: {:?}", err)).into()
//    }
//}
//
//impl<T> From<futures::future::ExecuteError<T>> for Error {
//    fn from(_: futures::future::ExecuteError<T>) -> Self {
//        MediaError::ExecuteError.into()
//    }
//}
