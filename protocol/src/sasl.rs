/// Compute the auth string using SASL PLAIN mechanism
pub fn plain_auth_string(username: &str, password: &str) -> String {
    format!("\0{}\0{}", username, password)
}
