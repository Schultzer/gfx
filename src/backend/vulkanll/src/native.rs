// Copyright 2017 The Gfx-rs Developers.
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

use vk;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShaderLib {
    pub inner: vk::ShaderModule,
}
unsafe impl Send for ShaderLib {}
unsafe impl Sync for ShaderLib {}

#[derive(Clone, Debug, Hash)]
pub struct PipelineSignature {
    pub layout: vk::PipelineLayout,
}
unsafe impl Send for PipelineSignature {}
unsafe impl Sync for PipelineSignature {}

#[derive(Clone, Debug, Hash)]
pub struct RenderPass {
    pub inner: vk::RenderPass,
}
unsafe impl Send for RenderPass {}
unsafe impl Sync for RenderPass {}

