use crate::{
    data_types::hash_out::{MerkleInput, HASH_OUT},
    poseidon2_hash::Poseidon2,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleNode {
    pub hash: MerkleInput,
    left: Option<Box<MerkleInput>>,
    right: Option<Box<MerkleInput>>,
}

pub fn build_merkle_tree(leaves: &[MerkleInput]) -> MerkleNode {
    let mut nodes: Vec<MerkleNode> = leaves
        .iter()
        .map(|leaf| MerkleNode {
            hash: *leaf,
            left: None,
            right: None,
        })
        .collect();
    while nodes.len() > 1 {
        let mut new_nodes = Vec::new();
        for i in 0..nodes.len() / 2 {
            let left = nodes[2 * i].clone();
            let right = nodes[2 * i + 1].clone();

            let mut inputs = Vec::with_capacity(left.hash.len() + right.hash.len());
            inputs.extend(left.hash);
            inputs.extend(right.hash);
            let combined_hash = Poseidon2::<HASH_OUT>::hash(&inputs);
            new_nodes.push(MerkleNode {
                hash: combined_hash,
                left: Some(Box::new(left.hash)),
                right: Some(Box::new(right.hash)),
            });
        }
        nodes = new_nodes;
    }
    nodes[0].clone()
}
