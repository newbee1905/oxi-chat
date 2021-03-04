#[derive(Clone, Debug)]
pub struct Argon {
	secret: String,
}

impl Argon {
	pub fn new(secret: String) -> Self {
		Self {
			secret: secret.to_owned(),
		}
	}

	pub fn hasher(&self) -> argonautica::Hasher<'static> {
		let mut hasher = argonautica::Hasher::default();
		let hasher = hasher.with_secret_key(&self.secret);
		hasher.to_owned()
	}

	pub fn verifier(&self) -> argonautica::Verifier<'static> {
		let mut verifier = argonautica::Verifier::default();
		let verifier = verifier.with_secret_key(&self.secret);
		verifier.to_owned()
	}
}
