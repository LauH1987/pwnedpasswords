use sha1::{Digest, Sha1};

pub trait Sha1Hash {
    fn sha1_hash(&self) -> String;
}

impl Sha1Hash for str {
    #[inline]
    fn sha1_hash(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(self);
        hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02X}", b))
            .fold(String::with_capacity(40), |mut acc, s| {
                acc.push_str(&s);
                acc
            })
    }
}
