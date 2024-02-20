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

//! Provide JS-Rust API bindings to create and encode Pact contract
use trn_pact::types::{
    opcode::{Comparator, Conjunction, OpCode, OpIndices},
    Contract, DataTable, Numeric, PactType, StringLike,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// A js handle for a rust versioned pact contract struct
#[wasm_bindgen(js_name = Pact)]
pub struct ContractJS(Contract);

#[wasm_bindgen(js_class = Pact)]
#[allow(irrefutable_let_patterns)]
impl ContractJS {
    #[wasm_bindgen(constructor)]
    pub fn new(data_table: JsValue, bytecode: JsValue) -> Self {
        console_error_panic_hook::set_once();

        let bytecode: Vec<u8> =
            serde_wasm_bindgen::from_value(bytecode).expect("Deserialization of bytecode failed");
        let input_data_table: Vec<String> = serde_wasm_bindgen::from_value(data_table)
            .expect("Deserialization of data_table failed");

        let mut data_table = Vec::<PactType>::new();
        // TODO: Find a better way to take input data table
        for item in input_data_table {
            match item.parse::<u64>() {
                Ok(number) => data_table.push(PactType::Numeric(Numeric(number))),
                Err(_) => data_table.push(PactType::StringLike(StringLike(item.into_bytes()))),
            }
        }

        let pact_contract = Contract {
            data_table: DataTable::new(data_table),
            bytecode,
        };
        ContractJS(pact_contract)
    }

    #[allow(non_snake_case)]
    /// Return the bytecode
    pub fn getBytecode(&self) -> JsValue {
        return serde_wasm_bindgen::to_value(&self.0.bytecode).unwrap();
    }

    /// encode the Pact contract
    pub fn encode(&self) -> Vec<u8> {
        let mut payload = vec![];
        self.0.encode(&mut payload);
        payload
    }
}

#[wasm_bindgen(js_name = LoadSource)]
pub enum LoadSourceJS {
    Input,
    DataTable,
}

#[wasm_bindgen(js_name = OpLoad)]
pub enum OpLoadJS {
    InputVsUser,
    InputVsInput,
}

#[wasm_bindgen(js_name = OpComp)]
pub enum OpCompJS {
    EQ,
    GT,
    GTE,
    IN,
}

#[wasm_bindgen(js_name = OpConj)]
pub enum OpConjJS {
    AND,
    OR,
    XOR,
}

#[wasm_bindgen(js_name = OpCodeComparator)]
pub struct OpCodeComparator(Comparator);

#[wasm_bindgen(js_class = OpCodeComparator)]
#[allow(irrefutable_let_patterns)]
impl OpCodeComparator {
    #[wasm_bindgen(constructor)]
    pub fn new(load: JsValue, op: JsValue, lhs: JsValue, rhs: JsValue, invert: JsValue) -> Self {
        console_error_panic_hook::set_once();
        let load: u8 =
            serde_wasm_bindgen::from_value(load).expect("Deserialization of load failed");
        let op: u8 = serde_wasm_bindgen::from_value(op).expect("Deserialization of op failed");
        let lhs: u8 = serde_wasm_bindgen::from_value(lhs).expect("Deserialization of lhs failed");
        let rhs: u8 = serde_wasm_bindgen::from_value(rhs).expect("Deserialization of rhs failed");
        let invert: bool =
            serde_wasm_bindgen::from_value(invert).expect("Deserialization of invert failed");

        OpCodeComparator(Comparator {
            load: load.into(),
            op: op.into(),
            indices: OpIndices { lhs, rhs },
            invert,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let payload: u8 = OpCode::COMP(self.0).into();
        // add indices
        let indice: u8 = (self.0.indices.lhs << 4) | self.0.indices.rhs;
        vec![payload, indice]
    }
}

#[wasm_bindgen(js_name = OpCodeConjunction)]
pub struct OpCodeConjunction(Conjunction);

#[wasm_bindgen(js_class = OpCodeConjunction)]
#[allow(irrefutable_let_patterns)]
impl OpCodeConjunction {
    #[wasm_bindgen(constructor)]
    pub fn new(op: JsValue, invert: JsValue) -> Self {
        console_error_panic_hook::set_once();
        let op: u8 = serde_wasm_bindgen::from_value(op).expect("Deserialization of op failed");
        let invert: bool =
            serde_wasm_bindgen::from_value(invert).expect("Deserialization of invert failed");

        OpCodeConjunction(Conjunction {
            op: op.into(),
            invert,
        })
    }

    pub fn encode(self) -> Vec<u8> {
        let payload: u8 = OpCode::CONJ(self.0).into();
        vec![payload]
    }
}
