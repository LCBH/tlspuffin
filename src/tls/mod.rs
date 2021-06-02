use ring::test::rand::FixedByteRandom;
use rustls::SupportedKxGroup;
use rustls::kx::KeyExchange;

mod tests;
pub mod op_impl;

pub fn deterministic_key_exchange(skxg: &'static SupportedKxGroup) -> KeyExchange {
    let random = FixedByteRandom { byte: 42 };
    //let random = SystemRandom::new();
    let ours =
        ring::agreement::EphemeralPrivateKey::generate(skxg.agreement_algorithm, &random).unwrap();

    let pubkey = ours.compute_public_key().unwrap();

    KeyExchange {
        skxg,
        privkey: ours,
        pubkey,
    }
}
