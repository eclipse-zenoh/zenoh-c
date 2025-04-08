//
// Copyright (c) 2017, 2022 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//
pub use sample_closure::*;
mod sample_closure;

pub use query_closure::*;
mod query_closure;

pub use reply_closure::*;
mod reply_closure;

pub use zenohid_closure::*;
mod zenohid_closure;

pub use response_channel::*;
mod response_channel;

pub use query_channel::*;
mod query_channel;

pub use sample_channel::*;
mod sample_channel;

pub use hello_closure::*;
mod hello_closure;

pub use log_closure::*;
mod log_closure;

#[cfg(feature = "unstable")]
pub use matching_status_closure::*;
#[cfg(feature = "unstable")]
mod matching_status_closure;

#[cfg(feature = "unstable")]
pub use miss_closure::*;
#[cfg(feature = "unstable")]
mod miss_closure;

use flume::{Receiver, Sender};

pub type SgNotifier = Sender<()>;

pub(crate) struct SyncObj<T: Sized> {
    pub(crate) value: T,
    _notifier: SgNotifier,
}

impl<T> SyncObj<T> {
    pub(crate) fn new(value: T, notifier: SgNotifier) -> Self {
        Self {
            value: value,
            _notifier: notifier,
        }
    }
}

pub(crate) struct SyncGroup {
    waiter: Receiver<()>,
    notifier: Option<SgNotifier>,
}

impl SyncGroup {
    pub(crate) fn new() -> SyncGroup {
        let (notifier, waiter) = flume::bounded(0);
        SyncGroup {
            waiter,
            notifier: Some(notifier),
        }
    }

    pub(crate) fn notifier(&self) -> SgNotifier {
        self.notifier.as_ref().unwrap().clone()
    }
}

impl Drop for SyncGroup {
    fn drop(&mut self) {
        self.notifier.take();
        self.waiter.recv().unwrap_err();
    }
}
