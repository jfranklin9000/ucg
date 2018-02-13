// Copyright 2017 Jeremy Wall <jeremy@marzhillstudios.com>
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

//! Errors for use by the ucg compiler.
use std::error;
use std::fmt;

use ast::*;

/// ErrorType defines the various types of errors that can result from compiling UCG into an
/// output format.
pub enum ErrorType {
    // Build Errors
    TypeFail,
    DuplicateBinding,
    IncompleteParsing,
    Unsupported,
    NoSuchSymbol,
    BadArgLen,
    FormatError,
    // Parsing Errors
    UnexpectedToken,
    EmptyExpression,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            &ErrorType::TypeFail => "TypeFail",
            &ErrorType::DuplicateBinding => "DuplicateBinding",
            &ErrorType::IncompleteParsing => "IncompleteParsing",
            &ErrorType::Unsupported => "Unsupported",
            &ErrorType::NoSuchSymbol => "NoSuchSymbol",
            &ErrorType::BadArgLen => "BadArgLen",
            &ErrorType::FormatError => "FormatError",
            &ErrorType::UnexpectedToken => "UnexpectedToken",
            &ErrorType::EmptyExpression => "EmptyExpression",
        };
        w.write_str(name)
    }
}

/// Error defines an Error type for parsing and building UCG code.
pub struct Error {
    pub err_type: ErrorType,
    pub pos: Position,
    pub msg: String,
    _pkgonly: (),
}

impl Error {
    pub fn new<S: Into<String>>(msg: S, t: ErrorType, pos: Position) -> Self {
        Error {
            err_type: t,
            pos: pos,
            msg: msg.into(),
            _pkgonly: (),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(
            w,
            "{}: \"{}\" {}:{}",
            self.err_type,
            self.msg,
            self.pos.line,
            self.pos.column
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(
            w,
            "{}: \"{}\" {}:{}",
            self.err_type,
            self.msg,
            self.pos.line,
            self.pos.column
        )
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}