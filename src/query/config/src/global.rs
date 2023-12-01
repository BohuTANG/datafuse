// Copyright 2021 Datafuse Labs
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

use std::sync::Arc;

use common_base::base::SingletonInstance;
use common_exception::Result;

use crate::InnerConfig;

pub struct GlobalConfig;

impl GlobalConfig {
    pub fn init(config: InnerConfig) -> Result<()> {
        SingletonInstance::set(Arc::new(config));
        Ok(())
    }

    pub fn instance() -> Arc<InnerConfig> {
        SingletonInstance::get()
    }

    pub fn try_get_instance() -> Option<Arc<InnerConfig>> {
        SingletonInstance::try_get()
    }
}
