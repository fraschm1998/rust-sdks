// Copyright 2023 LiveKit, Inc.
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

use crate::proto;
use crate::{FfiAsyncId, FfiHandleId};

pub mod audio_frame;
pub mod participant;
pub mod publication;
pub mod room;
pub mod track;
pub mod video_frame;

impl From<FfiHandleId> for proto::FfiHandleId {
    fn from(id: FfiHandleId) -> Self {
        Self { id: id as u64 }
    }
}

impl From<FfiAsyncId> for proto::FfiAsyncId {
    fn from(id: FfiAsyncId) -> Self {
        Self { id: id as u64 }
    }
}
