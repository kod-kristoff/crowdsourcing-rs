use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};

#[derive(Debug,Clone,PartialEq, Eq,PartialOrd, Ord, Hash)]
pub struct User {
    id: uuid::Uuid,
    /// UTC timestamp of the user when it's created.
    created: DateTime<Utc>
    email_addr: EmailAddress,
    // Name of the user (this is used as the nickname).
    name: UserName,
    // Fullname of the user.
    fullname: FullName,
    // Language used by the user in the PYBOSSA server.
    // locale: String,
}

impl User {
    pub fn new(email_addr: EmailAddress, name: UserName, fullname: FullName) -> Self {
        Self { id: uuid::Uuid::now_v7(), created: Utc::now(), email_addr, name, fullname }
    }
}

#[derive(Debug,Clone,PartialEq, Eq,PartialOrd, Ord, Hash)]
pub struct UserName(String);

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum UserNameError {
    #[error("User name cannot be empty")]
    Empty,
}

impl UserName {
    pub fn new(raw: &str) -> Result<Self, UserNameError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(UserNameError::Empty)
        } else {
            Ok(UserName(trimmed.to_string()))
        }
    }
}

impl Display for UserName  {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug,Clone,PartialEq, Eq,PartialOrd, Ord, Hash)]
pub struct FullName(String);

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum FullNameError {
    #[error("Full name cannot be empty")]
    Empty,
}

impl FullName {
    pub fn new(raw: &str) -> Result<Self, FullNameError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(FullNameError::Empty)
        } else {
            Ok(FullName(trimmed.to_string()))
        }
    }
}
#[derive(Debug,Clone,PartialEq, Eq,PartialOrd, Ord, Hash)]
pub struct EmailAddress(String);

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum EmailAddressError {
    #[error("Email address cannot be empty")]
    Empty,
}

impl EmailAddress    {
    pub fn new(raw: &str) -> Result<Self, EmailAddressError> {
        let trimmed = raw.trim();
        Self::validate_email_address(trimmed)?;
        Ok(Self(trimmed.to_string()))
    }

    fn validate_email_address(trimmed: &str) -> Result<(),EmailAddressError> {
        if trimmed.is_empty() {
            return Err(EmailAddressError::Empty);
        }
        Ok(())
    }
}

impl Display for EmailAddress  {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug,Clone,PartialEq, Eq,PartialOrd, Ord, Hash)]
pub struct CreateUserRequest {
    username: UserName,
    email_addr: EmailAddress,
    fullname: FullName,
}

impl CreateUserRequest {
    pub fn new(username: UserName, email_addr: EmailAddress, fullname: FullName) -> Self {
        Self { username, email_addr, fullname }
    }
}

#[derive(Debug,thiserror::Error,miette::Diagnostic)]
pub struct CreateUserError {
    #[error("User name '{0}' already exists")]
    DuplicateUserName(UserName),
    #[error("Email address '{0}' is already used")]
    DuplicateEmail(EmailAddress),
    #[error(transparent)]
    Unknown(miette::Report),
}
