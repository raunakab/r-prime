// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::fmt::{
    Display,
    Formatter,
    Result,
};

use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Clone)]
pub enum PortError {
    SenderDoesNotExist(String),
    SendError,
    PortNotConsumable,
}

impl Display for PortError {
    fn fmt(&self, _: &mut Formatter) -> Result {
        use PortError::*;

        match self {
            SenderDoesNotExist(_) => todo!(),
            SendError => todo!(),
            PortNotConsumable => todo!(),
        }
    }
}

impl<T> From<SendError<T>> for PortError {
    fn from(_: SendError<T>) -> Self { Self::SendError }
}
