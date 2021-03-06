pub use crate::msg::ErrorKind;
use hyper;
use std;
use std::fmt;
use std::io;
use url;

pub type FlyResult<T> = Result<T, FlyError>;

#[derive(Debug)]
pub struct FlyError {
  repr: Repr,
}

#[derive(Debug)]
enum Repr {
  Simple(ErrorKind, String),
  IoErr(io::Error),
  UrlErr(url::ParseError),
  HyperErr(hyper::Error),
}

pub fn new(kind: ErrorKind, msg: String) -> FlyError {
  FlyError {
    repr: Repr::Simple(kind, msg),
  }
}

impl FlyError {
  pub fn kind(&self) -> ErrorKind {
    match self.repr {
      Repr::Simple(kind, ref _msg) => kind,
      Repr::IoErr(ref err) => {
        use std::io::ErrorKind::*;
        match err.kind() {
          NotFound => ErrorKind::NotFound,
          PermissionDenied => ErrorKind::PermissionDenied,
          ConnectionRefused => ErrorKind::ConnectionRefused,
          ConnectionReset => ErrorKind::ConnectionReset,
          ConnectionAborted => ErrorKind::ConnectionAborted,
          NotConnected => ErrorKind::NotConnected,
          AddrInUse => ErrorKind::AddrInUse,
          AddrNotAvailable => ErrorKind::AddrNotAvailable,
          BrokenPipe => ErrorKind::BrokenPipe,
          AlreadyExists => ErrorKind::AlreadyExists,
          WouldBlock => ErrorKind::WouldBlock,
          InvalidInput => ErrorKind::InvalidInput,
          InvalidData => ErrorKind::InvalidData,
          TimedOut => ErrorKind::TimedOut,
          Interrupted => ErrorKind::Interrupted,
          WriteZero => ErrorKind::WriteZero,
          Other => ErrorKind::Other,
          UnexpectedEof => ErrorKind::UnexpectedEof,
          _ => unreachable!(),
        }
      }
      Repr::UrlErr(ref err) => {
        use url::ParseError::*;
        match err {
          EmptyHost => ErrorKind::EmptyHost,
          IdnaError => ErrorKind::IdnaError,
          InvalidPort => ErrorKind::InvalidPort,
          InvalidIpv4Address => ErrorKind::InvalidIpv4Address,
          InvalidIpv6Address => ErrorKind::InvalidIpv6Address,
          InvalidDomainCharacter => ErrorKind::InvalidDomainCharacter,
          RelativeUrlWithoutBase => ErrorKind::RelativeUrlWithoutBase,
          RelativeUrlWithCannotBeABaseBase => ErrorKind::RelativeUrlWithCannotBeABaseBase,
          SetHostOnCannotBeABaseUrl => ErrorKind::SetHostOnCannotBeABaseUrl,
          Overflow => ErrorKind::Overflow,
        }
      }
      Repr::HyperErr(ref err) => {
        // For some reason hyper::errors::Kind is private.
        if err.is_parse() {
          ErrorKind::HttpParse
        } else if err.is_user() {
          ErrorKind::HttpUser
        } else if err.is_canceled() {
          ErrorKind::HttpCanceled
        } else if err.is_closed() {
          ErrorKind::HttpClosed
        } else {
          ErrorKind::HttpOther
        }
      }
    }
  }
}

impl fmt::Display for FlyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.repr {
      Repr::IoErr(ref err) => err.fmt(f),
      Repr::UrlErr(ref err) => err.fmt(f),
      Repr::HyperErr(ref err) => err.fmt(f),
      Repr::Simple(_kind, ref s) => write!(f, "{}", s),
    }
  }
}

impl std::error::Error for FlyError {
  fn description(&self) -> &str {
    match self.repr {
      Repr::IoErr(ref err) => err.description(),
      Repr::UrlErr(ref err) => err.description(),
      Repr::HyperErr(ref err) => err.description(),
      Repr::Simple(_kind, ref s) => s.as_str(),
    }
  }

  fn cause(&self) -> Option<&std::error::Error> {
    match self.repr {
      Repr::IoErr(ref err) => Some(err),
      Repr::UrlErr(ref err) => Some(err),
      Repr::HyperErr(ref err) => Some(err),
      Repr::Simple(_, _) => None,
    }
  }
}

impl From<io::Error> for FlyError {
  #[inline]
  fn from(err: io::Error) -> FlyError {
    FlyError {
      repr: Repr::IoErr(err),
    }
  }
}

impl From<url::ParseError> for FlyError {
  #[inline]
  fn from(err: url::ParseError) -> FlyError {
    FlyError {
      repr: Repr::UrlErr(err),
    }
  }
}

impl From<hyper::Error> for FlyError {
  #[inline]
  fn from(err: hyper::Error) -> FlyError {
    FlyError {
      repr: Repr::HyperErr(err),
    }
  }
}

impl From<String> for FlyError {
  #[inline]
  fn from(err: String) -> FlyError {
    FlyError {
      repr: Repr::Simple(ErrorKind::String, err),
    }
  }
}

impl From<()> for FlyError {
  #[inline]
  fn from(_: ()) -> FlyError {
    FlyError {
      repr: Repr::Simple(
        ErrorKind::String,
        "Errored with non message error.".to_string(),
      ),
    }
  }
}

pub fn permission_denied() -> FlyError {
  new(ErrorKind::PermissionDenied, "permission denied".to_owned())
}
