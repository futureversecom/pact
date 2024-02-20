// Copyright 2023-2024 Futureverse Corporation Limited
// This file is part of Pact.
//
// Licensed under the Apache License v2.0;
// you may not use this file except in compliance with the License.
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// You should have received a copy of the Apache License v2.0
// along with Pact. If not, see:
//   <https://futureverse.com/licenses/apachev2.txt>

#![cfg_attr(not(feature = "std"), no_std)]

// interpreter can execute in `no_std` environment
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

pub mod interpreter;
pub mod types;
