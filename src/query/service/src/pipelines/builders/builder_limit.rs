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

use common_exception::Result;
use common_pipeline_core::processors::processor::ProcessorPtr;
use common_pipeline_transforms::processors::profile_wrapper::ProcessorProfileWrapper;
use common_sql::executor::Limit;

use crate::pipelines::processors::TransformLimit;
use crate::pipelines::PipelineBuilder;

impl PipelineBuilder {
    fn build_limit(&mut self, limit: &Limit) -> Result<()> {
        self.build_pipeline(&limit.input)?;

        if limit.limit.is_some() || limit.offset != 0 {
            self.main_pipeline.try_resize(1)?;
            return self.main_pipeline.add_transform(|input, output| {
                let transform =
                    TransformLimit::try_create(limit.limit, limit.offset, input, output)?;

                if self.enable_profiling {
                    Ok(ProcessorPtr::create(ProcessorProfileWrapper::create(
                        transform,
                        limit.plan_id,
                        self.proc_profs.clone(),
                    )))
                } else {
                    Ok(ProcessorPtr::create(transform))
                }
            });
        }
        Ok(())
    }
}
