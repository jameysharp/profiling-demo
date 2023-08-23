use fastly::http::StatusCode;
use fastly::{Error, Request, Response, SecretStore};
use jwt_simple::prelude::*;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let key = SecretStore::open("keys")?
        .get("signing-key")
        .expect("signing-key in keys store")
        .plaintext();
    let key = RS256KeyPair::from_pem(std::str::from_utf8(&key)?)?;

    let count = req
        .get_query_parameter("n")
        .and_then(|n| n.parse().ok())
        .unwrap_or(5);
    let data = if req.get_query_parameter("fast").is_some() {
        fib_fast(count)
    } else {
        fib_slow(count)
    };

    let claims = Claims::create(Duration::from_hours(2)).with_subject(data.to_string());
    let token = key.sign(claims)?;

    let mut resp = Response::from_status(StatusCode::OK);
    resp.set_body_text_plain(&format!("{}\n{}\n", data, token));
    Ok(resp)
}

fn fib_slow(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib_slow(n - 1) + fib_slow(n - 2)
}

fn fib_fast(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut last = 0;
    let mut result = 1;
    for _ in 2..=n {
        let next = last + result;
        last = result;
        result = next;
    }
    result
}
