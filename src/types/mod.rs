// Copyright 2019 Centrality Investments Limited
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

//!
//! Type definitions for the Pact interpreter and compiler
//!
mod base;
mod contract;
mod data_table;
pub mod opcode;
mod type_cast;

// Create nice top level exports
pub use base::{Numeric, PactType, StringLike};
pub use contract::{BinaryFormatErr, Contract};
pub use data_table::DataTable;
pub mod traits {
    pub use super::type_cast::IntoPact;
}
