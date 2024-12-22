use jwt_simple::prelude::*;

use crate::User;
type AppError = jwt_simple::Error;

const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISSUER: &str = "chat_server";
const JWT_AUDIENCE: &str = "chat_web";
// 加密
pub struct EncodingKeyPair(Ed25519KeyPair);

#[derive(Debug)]
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKeyPair {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        let key_pair = Ed25519KeyPair::from_pem(pem)?;
        Ok(Self(key_pair))
    }

    pub fn sign(&self, user: impl Into<User>) -> Result<String, AppError> {
        let claims = Claims::with_custom_claims(user.into(), Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(JWT_ISSUER).with_audience(JWT_AUDIENCE);
        self.0.sign(claims)
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        let key = Ed25519PublicKey::from_pem(pem)?;
        Ok(Self(key))
    }
    #[allow(unused)]
    pub fn verify(&self, token: &str) -> Result<User, AppError> {
        let options = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISSUER])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUDIENCE])),
            ..Default::default()
        };

        let claims = self.0.verify_token::<User>(token, Some(options))?;
        Ok(claims.custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn jwt_sign_verify_should_work() -> Result<()> {
        let encoding_pem = include_str!("../../fixtures/private.pem");
        let decoding_pem = include_str!("../../fixtures/public.pem");
        let ek = EncodingKeyPair::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;

        let user = User::new(1, "Tyr Chen", "tchen@acme.org");

        let token = ek.sign(user.clone())?;
        let user2 = dk.verify(&token)?;

        assert_eq!(user, user2);
        Ok(())
    }
}
