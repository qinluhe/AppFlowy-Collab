use crate::core::collab_plugin::CollabPlugin;
use crate::error::CollabError;

use collab_persistence::doc::YrsDoc;
use collab_persistence::CollabKV;

use std::sync::Arc;
use yrs::TransactionMut;

#[derive(Clone)]
pub struct CollabDiskPlugin {
    db: Arc<CollabKV>,
}
impl CollabDiskPlugin {
    pub fn new(db: Arc<CollabKV>) -> Result<Self, CollabError> {
        Ok(Self { db })
    }

    pub fn doc(&self) -> YrsDoc {
        self.db.doc()
    }
}

impl CollabPlugin for CollabDiskPlugin {
    fn did_init(&self, cid: &str, txn: &mut TransactionMut) {
        let doc = self.db.doc();
        if doc.is_exist(cid) {
            doc.load_doc(cid, txn).unwrap();
        } else {
            self.db.doc().insert_or_create_new_doc(cid, txn).unwrap();
        }
    }

    fn did_receive_update(&self, cid: &str, _txn: &TransactionMut, update: &[u8]) {
        self.db.doc().push_update(cid, update).unwrap();
    }
}
