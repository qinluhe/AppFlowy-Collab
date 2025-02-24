use crate::blocks::{BlockMap, TextMap};
use crate::error::DocumentError;
use collab::preclude::*;

const ROOT: &str = "document";
const BLOCKS: &str = "blocks";
const TEXTS: &str = "texts";

pub struct Document {
    #[allow(dead_code)]
    inner: Collab,
    root: MapRefWrapper,
    pub blocks: BlockMap,
    pub texts: TextMap,
}

impl Document {
    pub fn create(collab: Collab) -> Self {
        let (root, blocks, texts) = collab.with_transact_mut(|txn| {
            let root = collab
                .get_map_with_txn(txn, vec![ROOT])
                .unwrap_or_else(|| collab.create_map_with_txn(txn, ROOT));
            let blocks = collab
                .get_map_with_txn(txn, vec![ROOT, BLOCKS])
                .unwrap_or_else(|| root.insert_map_with_txn(txn, BLOCKS));
            let texts = collab
                .get_map_with_txn(txn, vec![ROOT, TEXTS])
                .unwrap_or_else(|| root.insert_map_with_txn(txn, TEXTS));
            (root, blocks, texts)
        });
        let blocks = BlockMap::new(blocks);
        let texts = TextMap::new(texts);
        Self {
            inner: collab,
            root,
            blocks,
            texts,
        }
    }

    pub fn to_json(&self) -> Result<String, DocumentError> {
        Ok(self.root.to_json())
    }
}
