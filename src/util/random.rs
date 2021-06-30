use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;
use tracing::info;

pub fn random_string(len: i8) -> String {
    let string: String = iter::repeat(())
        .map(|()| thread_rng().sample(Alphanumeric))
        .map(char::from)
        .take(len as usize)
        .collect();
    info!("random string : {:?}", string);
    string
}
