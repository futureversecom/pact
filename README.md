# Pact

An embedded contract DSL and toolchain for doughnuts in the TRN permission domain.  


Pact contracts are written in a simple bytecode and execute against dynamic input data to ensure their invariants are upheld.  
It is designed for integration with the TRN blockchain runtime to enable safe, powerful delegated transacitons.

It additionally supports a high-level english like language and compiler. This allows writing human readable "pacts" which the toolchain can interpret; achieving the notion of [Ricardian](https://en.wikipedia.org/wiki/Ricardian_contract).  

![alt text](https://github.com/futureversecom/pact/blob/main/design/pact-overview.png)

