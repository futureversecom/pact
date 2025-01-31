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

use crate::types::PactType;
use alloc::vec::Vec;
use bit_reverse::ParallelReverse;

/// A pact contract's static data table
#[cfg_attr(feature = "std", derive(PartialEq, Debug))]
pub struct DataTable(Vec<PactType>);

impl DataTable {
    /// Create a new `DataTable` with `values`
    pub fn new(values: Vec<PactType>) -> Self {
        Self { 0: values }
    }
    /// Push a PactType value into the table
    pub fn push(&mut self, val: PactType) {
        self.0.push(val);
    }
    /// Encode the data table
    pub fn encode(&self, buf: &mut Vec<u8>) {
        buf.push((self.0.len() as u8).swap_bits());
        for t in self.0.iter() {
            t.encode(buf);
        }
    }
    /// Decode a DataTable from `buf`.
    /// Return the DataTable and # of bytes read or error on failure.
    pub fn decode(buf: Vec<u8>) -> Result<(Self, usize), &'static str> {
        let mut table = DataTable(Default::default());
        let mut offset: usize = 1;
        let l = buf[0].swap_bits();
        for _ in 0..l {
            let (pact_type, read) = PactType::decode(buf[offset..].to_vec())?;
            table.push(pact_type);
            offset += read;
        }
        Ok((table, offset))
    }
}

impl AsRef<[PactType]> for DataTable {
    fn as_ref(&self) -> &[PactType] {
        &(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Numeric, StringLike};

    #[test]
    fn it_encodes() {
        let table = DataTable::new(vec![
            PactType::Numeric(Numeric(111)),
            PactType::Numeric(Numeric(333)),
            PactType::StringLike(StringLike(b"testing".to_vec())),
        ]);
        let mut encoded: Vec<u8> = Vec::new();
        table.encode(&mut encoded);

        // DataTable should simply encode to a concatenated list of it's encoded PactTypes
        let mut expected: Vec<u8> = vec![
            3, // length
            1, 8, 111, 0, 0, 0, 0, 0, 0, 0, // Numeric(111)
            1, 8, 77, 1, 0, 0, 0, 0, 0, 0, // Numeric(333)
        ];
        // StringLike("testing")
        expected.extend(&[0, 7]);
        expected = expected.into_iter().map(|b| b.swap_bits()).collect();
        expected.extend("testing".as_bytes());

        println!("expected: {:?}", expected);

        assert_eq!(encoded, expected,);
    }

    #[test]
    fn it_decodes() {
        let mut buf: Vec<u8> = vec![
            3, // lengths
            1, 8, 111, 0, 0, 0, 0, 0, 0, 0, // Numeric(111)
            1, 8, 77, 1, 0, 0, 0, 0, 0, 0, // Numeric(333)
        ];
        // StringLike("testing")
        buf.extend(&[0, 7]);
        buf = buf.into_iter().map(|b| b.swap_bits()).collect();
        buf.extend("testing".as_bytes());

        let expected = DataTable::new(vec![
            PactType::Numeric(Numeric(111)),
            PactType::Numeric(Numeric(333)),
            PactType::StringLike(StringLike(b"testing".to_vec())),
        ]);
        let (result, bytes_read) = DataTable::decode(buf.clone()).expect("it decodes");

        assert_eq!(result, expected);
        assert_eq!(bytes_read, buf.len() as usize);
    }
}
