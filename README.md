# Actix Web Demo
A small demo app to test the `actix-web` in `rust`.

Start the app using `cargo run -q`

```bash
λ cargo run -q
[2024-10-20T04:22:52Z INFO  actix_web_demo] staring server at http://127.0.0.1:8080
[2024-10-20T04:22:52Z INFO  actix_server::builder] starting 10 workers
[2024-10-20T04:22:52Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
[2024-10-20T04:22:52Z INFO  actix_server::server] starting service: "actix-web-service-127.0.0.1:8080", workers: 10, listening on: 127.0.0.1:8080
[2024-10-20T04:23:00Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET / HTTP/1.1" 200 458 "-" "HTTPie/3.0.2" 0.000373
```

There is a simple `html` form at the root to calculate `GCD` of two numbers that can be accessed by visiting `http://localhost:8080`.

A few restful operations are available as well:
```bash
λ http http://localhost:8080/countries
HTTP/1.1 200 OK
content-length: 497
content-type: application/json
date: Sun, 20 Oct 2024 04:26:03 GMT
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
x-version-id: 1.0

[
    {
        "country_code": "BR",
        "country_name": "Brazil"
    },
    {
        "country_code": "AQ",
        "country_name": "Antarctica"
    },
    {
        "country_code": "MY",
        "country_name": "Malaysia"
    },
    {
        "country_code": "US",
        "country_name": "United States of America"
    },
    {
        "country_code": "PH",
        "country_name": "Phillippines"
    },
    {
        "country_code": "IN",
        "country_name": "India"
    },
    {
        "country_code": "ID",
        "country_name": "Indonesia"
    },
    {
        "country_code": "RU",
        "country_name": "Russian"
    },
    {
        "country_code": "TH",
        "country_name": "Thailand"
    },
    {
        "country_code": "AT",
        "country_name": "Austria"
    }
]
```

`/status` operation:
```bash
λ http http://localhost:8080/status
HTTP/1.1 200 OK
content-length: 50
content-type: application/json
date: Sun, 20 Oct 2024 04:26:50 GMT
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
x-version-id: 1.0

{
    "message": "actix web service",
    "status": "success"
}

```

`/something` operation is for handling and testing the error scenarious and the one being tested has provides error non-deterinistically during each call:
```bash
λ http http://localhost:8080/something
HTTP/1.1 403 Forbidden
content-length: 0
date: Sun, 20 Oct 2024 04:27:29 GMT
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
x-version-id: 1.0

λ http http://localhost:8080/something
HTTP/1.1 200 OK
content-length: 40
date: Sun, 20 Oct 2024 04:28:59 GMT
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
x-version-id: 1.0

Nothing interesting happened. Try again.


```
