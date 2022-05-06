use std::convert::TryInto;

use rand::{distributions::Alphanumeric, rngs::OsRng, Rng as _};

use actix_session::storage::SessionKey;

pub fn generate_session_key() -> SessionKey {
    let value = std::iter::repeat(())
        .map(|()| OsRng.sample(Alphanumeric))
        .take(64)
        .collect::<Vec<_>>();

    String::from_utf8(value).unwrap().try_into().unwrap()
}
