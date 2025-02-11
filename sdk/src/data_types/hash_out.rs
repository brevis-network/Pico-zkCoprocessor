pub const HASH_OUT: usize = 8;

pub type MerkleInput = [u32; HASH_OUT];

pub trait HashBytes: Sized {
    // convert HashOut to be bytes
    fn to_be_bytes(&self) -> Vec<u8>;
}

impl HashBytes for MerkleInput {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.iter()
            .flat_map(|ic| ic.to_be_bytes())
            .collect()
    }
}
