// Copyright (C) 2023 Satoshi Konno All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Node represents a sever node.
pub struct Node {}

impl Node {
    pub fn new() -> Node {
        let node = Node {};
        node
    }

    pub fn start(&mut self) -> bool {
        true
    }

    pub fn stop(&mut self) -> bool {
        true
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.stop();
    }
}
