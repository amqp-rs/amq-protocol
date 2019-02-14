use cookie_factory::{do_gen, gen_be_u8, gen_copy, gen_slice, GenError};

/// Compute the auth string using SASL PLAIN mechanism
pub fn plain_auth_string(username: &str, password: &str) -> String {
    let sasl_plain_len = username.len() + password.len() + 2;
    let mut sasl_plain_creds = vec![0; sasl_plain_len];

    do_gen!((&mut sasl_plain_creds[..], 0),
        gen_be_u8!(0)                   >>
        gen_slice!(username.as_bytes()) >>
        gen_be_u8!(0)                   >>
        gen_slice!(password.as_bytes())
    ).expect("error serializing credentials");

    String::from_utf8_lossy(&sasl_plain_creds).to_string()
}
