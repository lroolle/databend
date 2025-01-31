// Copyright 2021 Datafuse Labs.
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

use std::fmt::Debug;
use std::sync::Arc;

use common_dal::InMemoryData;
use common_exception::Result;
use common_infallible::RwLock;

/// Datasource Table Context.
#[derive(Clone, Debug, Default)]
pub struct TableContext {
    pub in_memory_data: Arc<RwLock<InMemoryData<u64>>>,
}

impl TableContext {
    pub fn get_in_memory_data(&self) -> Result<Arc<RwLock<InMemoryData<u64>>>> {
        Ok(self.in_memory_data.clone())
    }
}
