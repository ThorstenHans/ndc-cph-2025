use crate::exports::fermyon::hmac::{sign, types::Error, verify};
use hmac::{Hmac, Mac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

wit_bindgen::generate!({
    world: "signing",
    path: "../wit/world.wit",
});

pub struct Component;

impl sign::Guest for Component {
    #[allow(async_fn_in_trait)]
    fn sign(data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, Error> {
        let mut mac =
            HmacSha256::new_from_slice(key.as_slice()).expect("HMAC can take key of any size");
        mac.update(data.as_slice());
        let result = mac.finalize();
        let b = result.into_bytes();
        Ok(hex::encode(b.to_vec()).as_bytes().to_vec())
    }
}

impl verify::Guest for Component {
    #[allow(async_fn_in_trait)]
    fn verify(data: Vec<u8>, key: Vec<u8>, signature: Vec<u8>) -> bool {
        let Ok(decoded) = hex::decode(&signature) else {
            return false;
        };
        let Ok(mut mac) = HmacSha256::new_from_slice(key.as_slice()) else {
            return false;
        };

        mac.update(data.as_slice());
        mac.verify_slice(decoded.as_slice()).is_ok()
    }
}

export!(Component);

#[cfg(test)]
mod tests {
    use crate::{
        Component,
        exports::fermyon::hmac::{sign::Guest as _, verify::Guest as _},
    };

    #[test]
    pub fn test_sign_and_verify() {
        let key = b"hello";
        let data = b"world";

        let signed = Component::sign(data.clone().to_vec(), key.clone().to_vec())
            .expect("Data should be signed");

        assert!(Component::verify(data.to_vec(), key.to_vec(), signed));
    }

    #[test]
    pub fn test_sign_modify_not_verify() {
        let key = b"hello";
        let data = b"world";

        let signed = Component::sign(data.clone().to_vec(), key.clone().to_vec())
            .expect("Data should be signed");

        let data = b"new-world";
        assert_eq!(
            false,
            Component::verify(data.to_vec(), key.to_vec(), signed)
        );
    }

    #[test]
    pub fn test_detect_signature_modification() {
        let key = b"hello";
        let data = b"world";

        let _ = Component::sign(data.clone().to_vec(), key.clone().to_vec())
            .expect("Data should be signed");
        let signature = b"abc";
        assert_eq!(
            false,
            Component::verify(data.to_vec(), key.to_vec(), signature.to_vec())
        );
    }
}
