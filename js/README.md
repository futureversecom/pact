# @therootnetwork/pact-nodejs

Wasm pact js interface.

## Create a Pact contract

```js
const {OpLoad, OpComp, OpConj} = require(".@therootnetwork/pact-nodejs");
const Pact = require('@therootnetwork/pact-nodejs').Pact;
const OpCodeComparator = require('@therootnetwork/pact-nodejs').OpCodeComparator;
const OpCodeConjunction = require('@therootnetwork/pact-nodejs').OpCodeConjunction;


let data_table = ["10","20"];
let comp = new OpCodeComparator(OpLoad.InputVsUser, OpComp.EQ,0,0,false);
let conj = new OpCodeConjunction(OpConj.AND,false);
let bytecode = new Uint8Array([
    ...comp.encode(),
    conj.encode(),
]);

return new Pact(data_table, bytecode);
```

## Encode and serialize a Pact contract

```js
const {OpLoad, OpComp, OpConj} = require(".@therootnetwork/pact-nodejs");
const Pact = require('@therootnetwork/pact-nodejs').Pact;
const OpCodeComparator = require('@therootnetwork/pact-nodejs').OpCodeComparator;
const OpCodeConjunction = require('@therootnetwork/pact-nodejs').OpCodeConjunction;


let data_table = ["10","20"];
let comp = new OpCodeComparator(OpLoad.InputVsUser, OpComp.EQ,0,0,false);
let conj = new OpCodeConjunction(OpConj.AND,false);
let bytecode = new Uint8Array([
    ...comp.encode(),
    conj.encode(),
]);

let pactContract = new Pact(data_table, bytecode);
return pactContract.encode();
```
