# Solana Certora Verification Tool (CVT) #

This workspace defines a set of functions and macros that can be called from Solana
programs to create non-deterministic environments, assume/assert, and
alternative (simpler but complete) implementations for std containers such as `Vec` class.

It includes the following crates:
- `vectors` [TODO: description]
- `cvt` [TODO: description]
- `hook_macro` - Defines macros for inserting hooks at the beginning and end of functions.
- `nondet` [TODO: description]
- `solana_cvt` [TODO: description]
- `stubs` [TODO: description]
- `calltrace` - Defines macros to print values in the Solana's prover calltrace
