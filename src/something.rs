/*
The  goal of  this example  is to  show how  to propagate  a
custom error type,  to a web handler that  will evaluate the
type  of error  that was  raised and  return an  appropriate
HTTPResponse.

This  example  uses a  50/50  chance  of returning  200  Ok,
otherwise one of  four possible http errors  will be chosen,
each with an equal chance of being selected:

    1. 403 Forbidden
    2. 401 Unauthorized
    3. 500 InternalServerError
    4. 400 BadRequest
*/

use actix_web::{Error, HttpResponse, ResponseError, Result};
use derive_more::Display;
use rand::{distributions::{Distribution, Standard}, thread_rng, Rng};

#[derive(Debug, Display)]
pub enum CustomError {
    #[display("Custom Error 1")]
    CustomOne,
    #[display("Custom Error 2")]
    CustomTwo,
    #[display("Custom Error 3")]
    CustomThree,
    #[display("Custom Error 4")]
    CustomFour,
}

impl Distribution<CustomError> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> CustomError {
        match rng.gen_range(0..4) {
            0 => CustomError::CustomOne,
            1 => CustomError::CustomTwo,
            2 => CustomError::CustomThree,
            _ => CustomError::CustomFour,
        }
    }
}

/// Actix Web uses `ResponseError` for conversion of errors to a response
impl ResponseError for CustomError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            CustomError::CustomOne => {
                println!("do some stuff related to CustomOne error");
                HttpResponse::Forbidden().finish()
            }
            CustomError::CustomTwo => {
                println!("do some stuff related to CustomTwo error");
                HttpResponse::Forbidden().finish()
            }
            CustomError::CustomThree => {
                println!("do some stuff related to CustomThree error");
                HttpResponse::Forbidden().finish()
            }
            _ => {
                println!("do some stuff related to CustomFour error");
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

/// randomly returns either () or one of the 4 CustomError variants
async fn perform_something_random() -> Result<(), CustomError> {
    let mut rng = thread_rng();

    // 30% chance that () will be returned by this function
    if rng.gen_bool(3.0/10.0) {
        Ok(())
    } else {
        Err(rand::random::<CustomError>())
    }
}

pub async fn perform_something() -> Result<HttpResponse, Error> {
    perform_something_random().await?;

    Ok(HttpResponse::Ok().body("Nothing interesting happened. Try again."))
}
