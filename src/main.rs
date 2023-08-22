use fastly::http::StatusCode;
use fastly::{Error, Request, Response};
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::{EncodePublicKey, LineEnding};
use rsa::rand_core::OsRng;
use rsa::sha2::Sha256;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::RsaPrivateKey;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let count = req
        .get_query_parameter("n")
        .and_then(|n| n.parse().ok())
        .unwrap_or(5);
    let data = format!(
        "{}",
        if req.get_query_parameter("fast").is_some() {
            fib_fast(count)
        } else {
            fib_slow(count)
        }
    );

    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
    let public_key = private_key.to_public_key();
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let signature = signing_key.sign_with_rng(&mut rng, data.as_bytes());

    let mut resp = Response::from_status(StatusCode::OK);
    resp.set_body_text_plain(&format!(
        "{}\n{:?}\n{}",
        data,
        signature.to_bytes(),
        public_key.to_public_key_pem(LineEnding::LF)?
    ));
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
