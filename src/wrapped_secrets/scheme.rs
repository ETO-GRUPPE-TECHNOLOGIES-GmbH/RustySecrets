use crate::errors::*;
use crate::proto::wrapped::SecretProto;
use crate::proto::VersionProto;
use crate::sss::Sss;

use protobuf::Message;
use rand::Rng;

pub(crate) use crate::sss::Share;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct WrappedSecrets;

impl WrappedSecrets {
    /// Performs threshold k-out-of-n Shamir's secret sharing.
    pub fn split_secret<R: Rng>(
        &self,
        rng: &mut R,
        k: u8,
        n: u8,
        secret: &[u8],
        mime_type: Option<String>,
        sign_shares: bool,
    ) -> Result<Vec<Share>> {
        let mut rusty_secret = SecretProto::new();
        rusty_secret.set_version(VersionProto::INITIAL_RELEASE);
        rusty_secret.set_secret(secret.to_owned());

        if let Some(mt) = mime_type {
            rusty_secret.set_mime_type(mt);
        }

        let data = rusty_secret.write_to_bytes().unwrap();

        Sss.split_secret(rng, k, n, data.as_slice(), sign_shares)
    }

    /// Recovers the secret from a k-out-of-n Shamir's secret sharing.
    ///
    /// At least `k` distinct shares need to be provided to recover the share.
    pub fn recover_secret(shares: Vec<Share>, verify_signatures: bool) -> Result<SecretProto> {
        let secret = Sss::recover_secret(shares, verify_signatures)?;

        SecretProto::parse_from_bytes(secret.as_slice())
            .chain_err(|| ErrorKind::SecretDeserializationError)
    }
}
