const {OpLoad, OpComp, OpConj} = require("../libNode/pact");
const Pact = require('../libNode/pact').Pact;
const OpCodeComparator = require('../libNode/pact').OpCodeComparator;
const OpCodeConjunction = require('../libNode/pact').OpCodeConjunction;

describe("wasm pact", () => {
    test("can construct pact contract and encode", () => {

        let data_table = ["10","20"];
        let comp = new OpCodeComparator(OpLoad.INPUT_VS_USER, OpComp.EQ,0,0,false);
        let conj = new OpCodeConjunction(OpConj.AND,false);
        let bytecode = new Uint8Array([
            ...comp.encode(),
            conj.encode(),
        ]);

        let pactContract = new Pact(data_table, bytecode);
        expect(pactContract.getBytecode()).toMatchObject(bytecode);
        let encoded_pact = pactContract.encode();
    });

    test("pact contract encode same as rust output 1", () => {
        let expected_payload = new Uint8Array([
            0, 64, 128, 16, 80, 0, 0, 0, 0, 0, 0, 0, 128, 16, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]);
        let data_table = ["10","20"];
        let comp = new OpCodeComparator(OpLoad.INPUT_VS_USER, OpComp.EQ,0,0,false);
        let bytecode = new Uint8Array([
            ...comp.encode(),
        ]);

        let pactContract = new Pact(data_table, bytecode);
        expect(pactContract.encode()).toEqual(expected_payload);
    });

    test("pact contract encode same as rust output 2", () => {
        let expected_payload = new Uint8Array([
            0, 64, 128, 16, 80, 0, 0, 0, 0, 0, 0, 0, 0, 48, 104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 0, 0, 0, 17
        ]);
        let data_table = ["10","hello, world"];
        let comp = new OpCodeComparator(OpLoad.INPUT_VS_USER, OpComp.EQ,0,0,false);
        let comp2 = new OpCodeComparator(OpLoad.INPUT_VS_USER, OpComp.EQ,1,1,false);
        let bytecode = new Uint8Array([
            ...comp.encode(),
            ...comp2.encode(),
        ]);

        let pactContract = new Pact(data_table, bytecode);
        expect(pactContract.encode()).toEqual(expected_payload);
    });

    test("pact contract encode same as rust output 3", () => {
        let expected_payload = new Uint8Array([
            0, 64, 128, 16, 80, 0, 0, 0, 0, 0, 0, 0, 0, 48, 104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 16, 0, 8, 17
        ]);
        let data_table = ["10","hello, world"];
        let comp = new OpCodeComparator(OpLoad.INPUT_VS_USER, OpComp.EQ,0,0,true);
        let comp2 = new OpCodeComparator(OpLoad.INPUT_VS_INPUT, OpComp.EQ,1,1,false);
        let bytecode = new Uint8Array([
            ...comp.encode(),
            ...comp2.encode(),
        ]);

        let pactContract = new Pact(data_table, bytecode);
        expect(pactContract.encode()).toEqual(expected_payload);
    });

    test("pact contract encode same as rust output 4", () => {
        let expected_payload = new Uint8Array([
            0, 64, 128, 16, 80, 0, 0, 0, 0, 0, 0, 0, 128, 16, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32
        ]);
        let data_table = ["10","20"];
        let comp = new OpCodeComparator(OpLoad.INPUT_VS_USER, OpComp.EQ,0,0,false);
        let conj = new OpCodeConjunction(OpConj.AND,false);
        let bytecode = new Uint8Array([
            ...comp.encode(),
            ...conj.encode(),
        ]);

        let pactContract = new Pact(data_table, bytecode);
        expect(pactContract.getBytecode()).toMatchObject(bytecode);

        expect(pactContract.encode()).toEqual(expected_payload);
    });
});
