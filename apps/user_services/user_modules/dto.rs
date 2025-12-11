use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::{Validate, ValidationError};
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateUserDto {
    #[validate(custom = "validate_email_strict")]
    pub email: String,
    #[validate(custom = "validate_name")]
    pub first_name: String,
    #[validate(custom = "validate_name")]
    pub last_name: String,
    pub password: String,
    #[validate(custom = "validate_phone_strict")]
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateUserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,

}

fn validate_email_strict(email: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !re.is_match(email) {
        let mut err = ValidationError::new("invalid_email");
        err.message = Some(std::borrow::Cow::Borrowed("invalid email format"));
        return Err(err);
    }
    Ok(())
}

fn validate_name(name: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^[A-Za-z\s\-]{2,}$").unwrap();
    if !re.is_match(name) {
        let mut err = ValidationError::new("invalid_name");
        err.message = Some(std::borrow::Cow::Borrowed("invalid name format"));
        return Err(err);
    }
    Ok(())
}

// fn validate_password_strong(password: &str) -> Result<(), ValidationError> {
//     if password.len() < 8 {
//         let mut err = ValidationError::new("password_too_short");
//         err.message = Some(std::borrow::Cow::Borrowed("password too short"));
//         return Err(err);
//     }

//     let mut has_lower = false;
//     let mut has_upper = false;
//     let mut has_digit = false;
//     let mut has_symbol = false;

//     for c in password.chars() {
//         if c.is_ascii_lowercase() {
//             has_lower = true;
//         } else if c.is_ascii_uppercase() {
//             has_upper = true;
//         } else if c.is_ascii_digit() {
//             has_digit = true;
//         } else {
//             has_symbol = true;
//         }
//     }

//     if !(has_lower && has_upper && has_digit && has_symbol) {
//         let mut err = ValidationError::new("weak_password");
//         err.message = Some(std::borrow::Cow::Borrowed("password is weak"));
//         return Err(err);
//     }

//     Ok(())
// }

fn validate_phone_strict(phone: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^\+?\d{10,15}$").unwrap();
    if !re.is_match(phone) {
        let mut err = ValidationError::new("invalid_phone");
        err.message = Some(std::borrow::Cow::Borrowed("invalid phone format"));
        return Err(err);
    }
    Ok(())
}
