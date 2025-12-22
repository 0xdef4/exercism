use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    key_offset: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        // todo!()
        Self {
            key: key.as_ref(),
            key_offset: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for (b, k) in data
            .iter_mut()
            .zip(self.key.iter().cycle().skip(self.key_offset))
        {
            *b ^= *k;
            self.key_offset += 1;
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        Data: IntoIterator<Item: Borrow<u8>>,
    {
        data.into_iter()
            .zip(self.key.iter().cycle().skip(self.key_offset))
            .map(|(e, k)| {
                self.key_offset += 1;
                e.borrow() ^ k
            })
    }
}
