use crate::model::User;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn generate_jwt(user: User) -> String {
	let secret = std::env::var("SECRET_KEY")
		.expect("SECRET_KEY must be set");
	let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())
		.expect("Failed to generate key.");

	let claims = BTreeMap::from([
		("id", user.id.to_string()),
		("name", user.name)
	]);

	claims.sign_with_key(&key)
		.expect("Failed to create jwt token")
}
