use std::path::PathBuf;

use tokio::sync::oneshot;
use uuid::Uuid;

use crate::index::{Document, SearchQuery, SearchResult, Settings};
use crate::index_controller::{Failed, IndexStats, Processed, Processing};

use super::{IndexMeta, IndexResult, IndexSettings};

pub enum IndexMsg {
    CreateIndex {
        uuid: Uuid,
        primary_key: Option<String>,
        ret: oneshot::Sender<IndexResult<IndexMeta>>,
    },
    Update {
        uuid: Uuid,
        meta: Processing,
        data: Option<std::fs::File>,
        ret: oneshot::Sender<IndexResult<Result<Processed, Failed>>>,
    },
    Search {
        uuid: Uuid,
        query: SearchQuery,
        ret: oneshot::Sender<anyhow::Result<SearchResult>>,
    },
    Settings {
        uuid: Uuid,
        ret: oneshot::Sender<IndexResult<Settings>>,
    },
    Documents {
        uuid: Uuid,
        attributes_to_retrieve: Option<Vec<String>>,
        offset: usize,
        limit: usize,
        ret: oneshot::Sender<IndexResult<Vec<Document>>>,
    },
    Document {
        uuid: Uuid,
        attributes_to_retrieve: Option<Vec<String>>,
        doc_id: String,
        ret: oneshot::Sender<IndexResult<Document>>,
    },
    Delete {
        uuid: Uuid,
        ret: oneshot::Sender<IndexResult<()>>,
    },
    GetMeta {
        uuid: Uuid,
        ret: oneshot::Sender<IndexResult<IndexMeta>>,
    },
    UpdateIndex {
        uuid: Uuid,
        index_settings: IndexSettings,
        ret: oneshot::Sender<IndexResult<IndexMeta>>,
    },
    Snapshot {
        uuid: Uuid,
        path: PathBuf,
        ret: oneshot::Sender<IndexResult<()>>,
    },
    GetStats {
        uuid: Uuid,
        ret: oneshot::Sender<IndexResult<IndexStats>>,
    },
}
