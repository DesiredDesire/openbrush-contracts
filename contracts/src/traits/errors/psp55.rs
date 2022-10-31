// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use super::{
    AccessControlError,
    OwnableError,
    PausableError,
    ReentrancyGuardError,
};
use openbrush::traits::String;

/// The PSP55 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP55Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Returned if recipient's address is zero.
    ZeroRecipientAddress,
    /// Returned if sender's address is zero.
    ZeroSenderAddress,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

impl From<OwnableError> for PSP55Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => PSP55Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => PSP55Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for PSP55Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => PSP55Error::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => PSP55Error::Custom(String::from("AC::RoleRedundant")),
            AccessControlError::InvalidCaller => PSP55Error::Custom(String::from("AC::InvalidCaller")),
        }
    }
}

impl From<PausableError> for PSP55Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => PSP55Error::Custom(String::from("P::Paused")),
            PausableError::NotPaused => PSP55Error::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for PSP55Error {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => PSP55Error::Custom(String::from("RG::ReentrantCall")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP55ReceiverError {
    TransferRejected(String),
}

impl From<PSP55ReceiverError> for PSP55Error {
    fn from(error: PSP55ReceiverError) -> Self {
        match error {
            PSP55ReceiverError::TransferRejected(message) => PSP55Error::SafeTransferCheckFailed(message),
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP55TokenTimelockError {
    PSP55Error(PSP55Error),
    /// Returned if the owner wants to withdraw the tokens before the release time
    CurrentTimeIsBeforeReleaseTime,
    /// Returned if there are no tokens to be released
    NoTokensToRelease,
    /// Returned if the timestamp provided is before the current time
    ReleaseTimeIsBeforeCurrentTime,
}

impl From<PSP55Error> for PSP55TokenTimelockError {
    fn from(error: PSP55Error) -> Self {
        match error {
            PSP55Error::Custom(message) => PSP55TokenTimelockError::PSP55Error(PSP55Error::Custom(message)),
            PSP55Error::InsufficientBalance => PSP55TokenTimelockError::PSP55Error(PSP55Error::InsufficientBalance),
            PSP55Error::InsufficientAllowance => PSP55TokenTimelockError::PSP55Error(PSP55Error::InsufficientAllowance),
            PSP55Error::ZeroRecipientAddress => PSP55TokenTimelockError::PSP55Error(PSP55Error::ZeroRecipientAddress),
            PSP55Error::ZeroSenderAddress => PSP55TokenTimelockError::PSP55Error(PSP55Error::ZeroSenderAddress),
            PSP55Error::SafeTransferCheckFailed(message) => {
                PSP55TokenTimelockError::PSP55Error(PSP55Error::SafeTransferCheckFailed(message))
            }
        }
    }
}

impl From<OwnableError> for PSP55TokenTimelockError {
    fn from(ownable: OwnableError) -> Self {
        PSP55TokenTimelockError::PSP55Error(ownable.into())
    }
}

impl From<AccessControlError> for PSP55TokenTimelockError {
    fn from(access: AccessControlError) -> Self {
        PSP55TokenTimelockError::PSP55Error(access.into())
    }
}

impl From<PausableError> for PSP55TokenTimelockError {
    fn from(pausable: PausableError) -> Self {
        PSP55TokenTimelockError::PSP55Error(pausable.into())
    }
}

impl From<ReentrancyGuardError> for PSP55TokenTimelockError {
    fn from(guard: ReentrancyGuardError) -> Self {
        PSP55TokenTimelockError::PSP55Error(guard.into())
    }
}
