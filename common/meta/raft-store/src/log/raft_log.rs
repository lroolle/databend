// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ops::RangeBounds;

use async_raft::raft::Entry;
use common_meta_sled_store::sled;
use common_meta_sled_store::AsKeySpace;
use common_meta_sled_store::SledTree;
use common_meta_types::LogEntry;
use common_meta_types::LogIndex;
use common_tracing::tracing;

use crate::config::RaftConfig;
use crate::sled_key_spaces::Logs;

const TREE_RAFT_LOG: &str = "raft_log";

/// RaftLog stores the logs of a raft node.
/// It is part of MetaStore.
pub struct RaftLog {
    pub(crate) inner: SledTree,
}

impl RaftLog {
    /// Open RaftLog
    #[tracing::instrument(level = "info", skip(db,config), fields(config_id=%config.config_id))]
    pub async fn open(db: &sled::Db, config: &RaftConfig) -> common_exception::Result<RaftLog> {
        tracing::info!(?config);

        let tree_name = config.tree_name(TREE_RAFT_LOG);
        let inner = SledTree::open(db, &tree_name, config.is_sync())?;
        let rl = RaftLog { inner };
        Ok(rl)
    }

    pub fn contains_key(&self, key: &LogIndex) -> common_exception::Result<bool> {
        self.logs().contains_key(key)
    }

    pub fn get(&self, key: &LogIndex) -> common_exception::Result<Option<Entry<LogEntry>>> {
        self.logs().get(key)
    }

    pub fn last(&self) -> common_exception::Result<Option<(LogIndex, Entry<LogEntry>)>> {
        self.logs().last()
    }

    /// Delete logs that are in `range`.
    ///
    /// When this function returns the logs are guaranteed to be fsync-ed.
    ///
    /// TODO(xp): in raft deleting logs may not need to be fsync-ed.
    ///
    /// 1. Deleting happens when cleaning applied logs, in which case, these logs will never be read:
    ///    The logs to clean are all included in a snapshot and state machine.
    ///    Replication will use the snapshot for sync, or create a new snapshot from the state machine for sync.
    ///    Thus these logs will never be read. If an un-fsync-ed delete is lost during server crash, it just wait for next delete to clean them up.
    ///
    /// 2. Overriding uncommitted logs of an old term by some new leader that did not see these logs:
    ///    In this case, atomic delete is quite enough(to not leave a hole).
    ///    If the system allows logs hole, non-atomic delete is quite enough(depends on the upper layer).
    ///
    pub async fn range_remove<R>(&self, range: R) -> common_exception::Result<()>
    where R: RangeBounds<LogIndex> {
        self.logs().range_remove(range, true).await
    }

    /// Returns an iterator of logs
    pub fn range<R>(
        &self,
        range: R,
    ) -> common_exception::Result<
        impl DoubleEndedIterator<Item = common_exception::Result<(LogIndex, Entry<LogEntry>)>>,
    >
    where
        R: RangeBounds<LogIndex>,
    {
        self.logs().range(range)
    }

    pub fn range_keys<R>(&self, range: R) -> common_exception::Result<Vec<LogIndex>>
    where R: RangeBounds<LogIndex> {
        self.logs().range_keys(range)
    }

    pub fn range_values<R>(&self, range: R) -> common_exception::Result<Vec<Entry<LogEntry>>>
    where R: RangeBounds<LogIndex> {
        self.logs().range_values(range)
    }

    /// Append logs into RaftLog.
    /// There is no consecutiveness check. It is the caller's responsibility to leave no holes(if it runs a standard raft:DDD).
    /// There is no overriding check either. It always overrides the existent ones.
    ///
    /// When this function returns the logs are guaranteed to be fsync-ed.
    pub async fn append(&self, logs: &[Entry<LogEntry>]) -> common_exception::Result<()> {
        self.logs().append_values(logs).await
    }

    /// Insert a single log.
    #[tracing::instrument(level = "debug", skip(self, log), fields(log_id=format!("{}",log.log_id).as_str()))]
    pub async fn insert(
        &self,
        log: &Entry<LogEntry>,
    ) -> common_exception::Result<Option<Entry<LogEntry>>> {
        self.logs().insert_value(log).await
    }

    /// Returns a borrowed key space in sled::Tree for logs
    fn logs(&self) -> AsKeySpace<Logs> {
        self.inner.key_space()
    }
}
