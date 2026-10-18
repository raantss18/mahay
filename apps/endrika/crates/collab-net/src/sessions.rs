//! The sessions a link carries at once (the engine has one per document): each under the id the
//! engine gave it, with its own [`Inbox`], so what happens in one never reaches another and a
//! session that ended can't reach the next. The desktop link and the browser link both keep
//! theirs here.

use std::collections::BTreeMap;

use figcraft_engine::live::{Event, SessionId};

use crate::inbox::{Inbox, Waker};

/// Running sessions of kind `T` (a host or a guest connection) by id.
pub(crate) struct Sessions<T> {
    wake: Waker,
    running: BTreeMap<SessionId, (T, Inbox)>,
}

impl<T> Sessions<T> {
    pub(crate) fn new(wake: Waker) -> Self {
        Self { wake, running: BTreeMap::new() }
    }

    /// A mailbox for a session about to start.
    pub(crate) fn inbox(&self) -> Inbox {
        Inbox::new(self.wake.clone())
    }

    /// Session `id` started, with the mailbox its threads fill. Returns the session that had
    /// the id, to be ended.
    pub(crate) fn insert(&mut self, id: SessionId, session: T, inbox: Inbox) -> Option<T> {
        self.running.insert(id, (session, inbox)).map(|(old, _)| old)
    }

    /// Session `id` is over: it comes back to be ended, and its mailbox is dropped.
    pub(crate) fn remove(&mut self, id: SessionId) -> Option<T> {
        self.running.remove(&id).map(|(session, _)| session)
    }

    pub(crate) fn get(&self, id: SessionId) -> Option<&T> {
        self.running.get(&id).map(|(session, _)| session)
    }

    /// Every session's id, oldest first.
    pub(crate) fn ids(&self) -> Vec<SessionId> {
        self.running.keys().copied().collect()
    }

    /// What happened in each session since the last call, with its id.
    pub(crate) fn poll(&self) -> Vec<(SessionId, Event)> {
        self.running.iter().flat_map(|(id, (_, inbox))| inbox.drain().into_iter().map(|e| (*id, e))).collect()
    }
}
