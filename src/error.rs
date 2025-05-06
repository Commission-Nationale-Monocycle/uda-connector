use std::fmt::Debug;
use log::error;
use crate::error::UdaError::MalformedSelector;
use scraper::error::SelectorErrorKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UdaError {
    #[error("The connection to UDA has failed.")]
    ConnectionFailed,
    #[error("The page content couldn't be read.")]
    CantReadPageContent,
    #[error("The organisation memberships is inaccessible.")]
    OrganizationMembershipsAccessFailed,
    #[error("Missing permissions to read the page")]
    LackOfPermissions,
    #[error("Wrong credentials to log into UDA")]
    WrongCredentials,
    #[error("Provided selector is malformed [selector: {0}]")]
    MalformedSelector(String),
    #[error("The member can't be marked as confirmed [id: {0}]")]
    MemberConfirmationFailed(u16),
    #[error("The exported XLS file is malformed")]
    MalformedXlsFile,
}

impl From<SelectorErrorKind<'_>> for UdaError {
    fn from(value: SelectorErrorKind<'_>) -> Self {
        MalformedSelector(value.to_string())
    }
}

pub fn log_error_and_return<E: Debug, T>(value_to_return: T) -> impl FnOnce(E) -> T {
    |e| {
        error!("{e:#?}");
        value_to_return
    }
}

pub fn log_message_and_return<E: Debug, T>(
    message: &str,
    value_to_return: T,
) -> impl FnOnce(E) -> T {
    move |e| {
        error!("{message}\n{e:#?}");
        value_to_return
    }
}