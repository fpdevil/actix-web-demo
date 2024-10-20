use core::fmt;

use std::collections::HashMap;

use actix_files::NamedFile;
use actix_web::{
    http::{
        self,
        header::ContentType,
    },
    web::{self, Form},
    HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GcdParameters {
    x: u64,
    y: u64,
}

#[derive(Serialize)]
struct GenericResponse {
    status: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct IFactors {
    factors: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
struct IFactorParams {
    number: u64,
}

#[derive(Debug, Deserialize)]
pub struct RouteParams {
    url: String,
}

#[derive(Serialize)]
pub struct Country {
    country_code: String,
    country_name: String,
}

impl Country {
    fn new(country_code: &str, country_name: &str) -> Self {
        Self {
            country_code: country_code.to_string(),
            country_name: country_name.to_string(),
        }
    }
}

impl fmt::Display for IFactors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;

        // iterate over `v` in the `vector` while enumerating the iteration
        for (index, v) in self.factors.iter().enumerate() {
            if index != 0 {
                // here, index is the count
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // now close the opened bracket and return fmt::Result type value
        write!(f, " ]")
        // Ok(())
    }
}

// calculate GCD of 2 numbers
fn gcd(mut x: u64, mut y: u64) -> u64 {
    assert!(x != 0 && y != 0);
    // based off euclidean division equation: a = b · q + r
    while y != 0 {
        // swap x & y
        if x > y {
            let temp = y;
            y = x;
            x = temp;
        }
        y %= x;
    }
    x
}

#[actix_web::post("/gcd")]
pub async fn post_gcd_handler(form: Form<GcdParameters>) -> impl Responder {
    if form.x == 0 || form.y == 0 {
        return HttpResponse::BadRequest()
            .content_type(http::header::ContentType::html())
            .body("<h2>Computing GCD with zero is boring...</h2>");
    }

    let response = format!(
        "<h2>The Greatest Common Divisor of numbers </b>{}</b> and <b>{}</b> is </b>{}</b></h2>\n",
        form.x,
        form.y,
        gcd(form.x, form.y)
    );

    HttpResponse::Ok()
        .content_type(http::header::ContentType::html())
        .body(response)
}

// favicon handler
#[actix_web::get("/favicon")]
pub async fn favicon_handler() -> Result<impl Responder> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

#[actix_web::head("/{other_url:.*}")]
pub async fn head_handler(other_url: web::Path<String>) -> impl Responder {
    let message: &str = "HEAD Not Allowed1";
    let response_json = &GenericResponse {
        status: other_url.to_string(),
        message: message.to_string(),
    };
    HttpResponse::MethodNotAllowed().json(response_json)
}

#[actix_web::get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

pub async fn health_check_handler() -> impl Responder {
    let message: &str = "actix web service";
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: message.to_string(),
    };

    HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .json(response_json)
}

pub async fn countries_handler() -> impl Responder {
    let mut vec: Vec<Country> = Vec::new();

    let cmap = HashMap::from([
        ("PH", "Phillippines"),
        ("IN", "India"),
        ("RU", "Russian"),
        ("TH", "Thailand"),
        ("US", "United States of America"),
        ("BR", "Brazil"),
        ("AT", "Austria"),
        ("AQ", "Antarctica"),
        ("MY", "Malaysia"),
        ("ID", "Indonesia"),
    ]);

    for (key, val) in cmap.iter() {
        vec.push(Country::new(key, val));
    }

    //web::Json(vec)
    HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .json(vec)
}

// calculate factors if a positive number
fn ifactors(number: u64) -> IFactors {
    let mut ifactors: Vec<u64> = vec![];

    for i in 1..((number as f64).sqrt() as u64 + 1) {
        if number % i == 0 {
            // push the smallest of the factors into the vector
            ifactors.push(i);

            if i != number / i {
                // push the largest of the factors into the vector
                ifactors.push(number / i);
            }
        }
    }

    ifactors.sort();

    IFactors { factors: ifactors }
}

#[actix_web::get("/factorize/{number}")]
pub async fn factorize_handler(form: web::Path<IFactorParams>) -> impl Responder {
    if form.number <= 0 {
        return HttpResponse::BadRequest()
            .content_type(ContentType::html())
            .body("<h3>Cannot calculate Factors of a Negative or Zero value</h3>");
    }
    let factors = ifactors(form.number);
    let response = format!(
        "<h3>Factors of {} are <p><span style=\"color:317399\">{}</span></p></h3>",
        form.number, factors,
    );

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(response)
}

#[actix_web::get("/redirect")]
pub async fn redirect_handler(params: web::Query<RouteParams>) -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", format!("{}", params.url)))
        .finish()
}

pub async fn index_handler() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <head>
                <meta charset="utf-8">
                <title>GCD Calculator</title>
            </head>
            <body>
                <h2>Calculate GCD of numbers</h2>
                <form action="/gcd" method="post">
                    <input type="text" name="x">
                    <input type="text" name="y">
                    <button type="submit">Compute GCD</button>
                </form>
            </body>
        "#,
    )
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 3 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
