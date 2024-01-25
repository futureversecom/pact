// Copyright 2022-2023 Futureverse Corporation Limited

//! Provide JS-Rust API bindings to create and inspect TRNNut
use pact::types::{
    opcode::{Comparator, Conjunction, OpCode, OpComp, OpIndices, OpLoad},
    Contract, DataTable, Numeric, PactType,
    PactType::{Numeric as NumericE, StringLike as StringLikeE},
    StringLike,
};
use parity_scale_codec::{Decode, Encode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[inline]
fn from_slice_32(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    if bytes.len() < 32 {
        log("expected 32 byte array");
        return array;
    }
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
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
    INPUT_VS_USER,
    INPUT_VS_INPUT,
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
        let mut payload: u8 = OpCode::COMP(self.0).into();
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

#[test]
fn test_OpCodeComparator_encode() {
    let opcomp = OpCodeComparator(Comparator {
        load: OpLoad::INPUT_VS_USER,
        op: OpComp::EQ,
        indices: OpIndices { lhs: 0, rhs: 0 },
        invert: false,
    });

    let encoded = opcomp.encode();
    println!("{:?}", encoded);
}
