/// This module implements a basic Ethereum Virtual Machine (EVM) opcode interpreter.
///
/// It defines gas costs for various operations, such as arithmetic, storage, and contract execution.
/// The `Opcode` struct encapsulates information about each opcode, including its name, number of inputs
/// and outputs, and associated gas costs.
///
/// The `new_opcodes` function initializes a mapping of opcodes to their corresponding operations, such
/// as ADD, SUB, CALL, and LOG. The stack operations include standard arithmetic functions, boolean
/// logic, and memory manipulation functions like MLOAD and MSTORE.
///
/// This interpreter handles gas calculation based on EVM specifications, allowing for the simulation
/// of smart contract execution.

/// The general gas cost function, C, for EVM operations is defined in the Yellow Paper as:
/// C(σ, μ, A, I) ≡ Cmem(μ′i) − Cmem(μi) plus additional costs associated with specific operations.
/// These operations, such as SELFDESTRUCT, are assigned different weights based on their computational costs.
/// For example, operations in Wzero (e.g., STOP, RETURN, REVERT) have lower costs, while those in Whigh (e.g., JUMPI) have higher costs.

use super::*;
// Constants representing gas prices for various operations in the Ethereum Virtual Machine (EVM).
// These values are derived from the Ethereum Yellow Paper and Ethereum Improvement Proposals (EIPs).
// They serve to quantify the computational cost of executing operations, ensuring fair resource allocation
// and preventing abuse of the network.

pub const GDEFAULT: usize = 1; // Default gas cost for basic operations.
pub const GMEMORY: usize = 3; // Gas cost for using memory (per 32 bytes).
pub const GQUADRATICMEMDENOM: usize = 512; // Cost factor for quadratic memory calculations (1 gas per 512 quadwords).
// Storage-related gas costs
const GSTORAGEREFUND: usize = 15000; // Gas refund amount for freeing up storage.
const GSTORAGEKILL: usize = 5000; // Gas cost for deleting storage.
const GSTORAGEMOD: usize = 5000; // Gas cost for modifying storage.
const GSTORAGEADD: usize = 20000; // Gas cost for adding new storage.
const NETSSTORENOOPGAS: u64 = 200;
const NETSSTOREINITGAS: u64 = 20000;
const NETSSTORECLEARREFUND: u64 = 15000;
const NETSSTORERESETCLEARREFUND: u64 = 19800;
const NETSSTORERESETREFUND: u64 = 4800;
const NETSSTORECLEANGAS: u64 = 5000;
const NETSSTOREDIRTYGAS: u64 = 200;
// Memory and copy-related gas costs
const GCOPY: usize = 3; // Gas cost for copying one 32-byte word.
const GEXPONENTBYTE: usize = 10; // Cost of the EXP exponent per byte.
const EXP_SUPPLEMENTAL_GAS: usize = 40; // Supplemental gas cost for EXP operations.

const GCONTRACTBYTE: usize = 200; // one byte of code in contract creation
const GCALLVALUETRANSFER: usize = 9000; // non-zero-valued call
const GLOGBYTE: usize = 8; // cost of a byte of logdata
// Transaction-related gas costs
const GTXCOST: usize = 21000; // TX BASE GAS COST
const GTXDATAZERO: usize = 4; // TX DATA ZERO BYTE GAS COST
const GTXDATANONZERO: usize = 68; // TX DATA NON ZERO BYTE GAS COST

const GSHA3WORD: usize = 6; // Cost of SHA3 per word
const GSHA256BASE: usize = 60; // Base c of SHA256
const GSHA256WORD: usize = 12; // Cost of SHA256 per word
const GRIPEMD160BASE: usize = 600; // Base cost of RIPEMD160
const GRIPEMD160WORD: usize = 120; // Cost of RIPEMD160 per word
const GIDENTITYBASE: usize = 15; // Base cost of indentity
const GIDENTITYWORD: usize = 3; // Cost of identity per word
const GECRECOVER: usize = 3000; // Cost of ecrecover op

const GSTIPEND: usize = 2300;

const GCALLNEWACCOUNT: usize = 25000;
const GSUICIDEREFUND: usize = 24000;

// Structure representing an opcode with its attributes,
/// - `name`: The name of the opcode.
/// - `inputs`: The number of items removed from the stack.
/// - `outputs`: The number of items added to the stack.
/// - `gas`: The amount of gas required to execute this opcode.
pub struct Opcode {
    pub name: String,
    pub inputs: u32,
    pub outputs: u32,
    pub gas: u64,
}

// Function to create a new Opcode instance
pub fn new_opcode(name: &str, inputs: u32, outputs: u32, gas: u64) -> Opcode {
    Opcode {
      name: name.to_string(),
      inputs,
      outputs,
      gas,
    }
}

// Function to create a hashmap of opcodes
pub fn new_opcodes() -> Hashmap<u8, Opcode> {
    let mut opcodes: Hashmap<u8,Opcode> = Hashmap::new();
    // 0s: Stop and Arithmetic Operations
    // All arithmetic is modulo 2^256 unless otherwise noted.
    opcodes.insert(0x00, new_opcode("STOP", 0, 0, 0));
    opcodes.insert(0x01, new_opcode("ADD", 2, 1, 3));
    opcodes.insert(0x02, new_opcode("MUL", 2, 1, 5));
    opcodes.insert(0x03, new_opcode("SUB", 2, 1, 3));
    opcodes.insert(0x04, new_opcode("DIV", 2, 1, 5));
    opcodes.insert(0x05, new_opcode("SDIV", 2, 1, 5));
    opcodes.insert(0x06, new_opcode("MOD", 2, 1, 5));
    opcodes.insert(0x07, new_opcode("SMOD", 2, 1, 5));
    opcodes.insert(0x08, new_opcode("ADDMOD", 3, 1, 8));
    opcodes.insert(0x09, new_opcode("MULMOD", 3, 1, 8));
    opcodes.insert(0x0a, new_opcode("EXP", 2, 1, 10));
    opcodes.insert(0x0b, new_opcode("SIGNEXTEND", 2, 1, 5));

    // 10s: Comparison & Bitwise Logic Operations
    opcodes.insert(0x10, new_opcode("LT", 2, 1, 3));
    opcodes.insert(0x11, new_opcode("GT", 2, 1, 3));
    opcodes.insert(0x12, new_opcode("SLT", 2, 1, 3));
    opcodes.insert(0x13, new_opcode("SGT", 2, 1, 3));
    opcodes.insert(0x14, new_opcode("EQ", 2, 1, 3));
    opcodes.insert(0x15, new_opcode("ISZERO", 1, 1, 3));
    opcodes.insert(0x16, new_opcode("AND", 2, 1, 3));
    opcodes.insert(0x17, new_opcode("OR", 2, 1, 3));
    opcodes.insert(0x18, new_opcode("XOR", 2, 1, 3));
    opcodes.insert(0x19, new_opcode("NOT", 1, 1, 3));
    opcodes.insert(0x1a, new_opcode("BYTE", 2, 1, 3));
    opcodes.insert(0x1b, new_opcode("SHL", 2, 1, 3));
    opcodes.insert(0x1c, new_opcode("SHR", 2, 1, 3));
    opcodes.insert(0x1d, new_opcode("SAR", 2, 1, 3));

    // 20s: KECCAK256
    opcodes.insert(0x20, new_opcode("KECCAK256", 2, 1, 30));

    // 30s: Environmental Information
    opcodes.insert(0x30, new_opcode("ADDRESS", 0, 1, 2));
    opcodes.insert(0x31, new_opcode("BALANCE", 1, 1, 100));
    opcodes.insert(0x32, new_opcode("ORIGIN", 0, 1, 2));
    opcodes.insert(0x33, new_opcode("CALLER", 0, 1, 2));
    opcodes.insert(0x34, new_opcode("CALLVALUE", 0, 1, 2));
    opcodes.insert(0x35, new_opcode("CALLDATALOAD", 1, 1, 3));
    opcodes.insert(0x36, new_opcode("CALLDATASIZE", 0, 1, 2));
    opcodes.insert(0x37, new_opcode("CALLDATACOPY", 3, 0, 3));
    opcodes.insert(0x38, new_opcode("CODESIZE", 0, 1, 2));
    opcodes.insert(0x39, new_opcode("CODECOPY", 3, 0, 3));
    opcodes.insert(0x3a, new_opcode("GASPRICE", 0, 1, 2));
    opcodes.insert(0x3b, new_opcode("EXTCODESIZE", 1, 1, 100));
    opcodes.insert(0x3c, new_opcode("EXTCODECOPY", 4, 0, 100));
    opcodes.insert(0x3d, new_opcode("RETURNDATASIZE", 0, 1, 2));
    opcodes.insert(0x3e, new_opcode("RETURNDATACOPY", 3, 0, 3));
    opcodes.insert(0x3f, new_opcode("EXTCODEHASH", 1, 1, 100));

    // 40s: Block Information
    opcodes.insert(0x40, new_opcode("BLOCKHASH", 1, 1, 20));
    opcodes.insert(0x41, new_opcode("COINBASE", 0, 1, 2));
    opcodes.insert(0x42, new_opcode("TIMESTAMP", 0, 1, 2));
    opcodes.insert(0x43, new_opcode("NUMBER", 0, 1, 2));
    opcodes.insert(0x44, new_opcode("PREVRANDAO", 0, 1, 2));
    opcodes.insert(0x45, new_opcode("GASLIMIT", 0, 1, 2));
    opcodes.insert(0x46, new_opcode("CHAINID", 0, 1, 2));
    opcodes.insert(0x47, new_opcode("SELFBALANCE", 0, 1, 5));
    opcodes.insert(0x48, new_opcode("BASEFEE", 0, 1, 2));

    // 50s: Stack, Memory, Storage and Flow Operations
    opcodes.insert(0x50, new_opcode("POP", 1, 0, 2));
    opcodes.insert(0x51, new_opcode("MLOAD", 1, 1, 3));
    opcodes.insert(0x52, new_opcode("MSTORE", 2, 0, 3));
    opcodes.insert(0x53, new_opcode("MSTORE8", 2, 0, 3));
    opcodes.insert(0x54, new_opcode("SLOAD", 1, 1, 100));
    opcodes.insert(0x55, new_opcode("SSTORE", 2, 0, 100));
    opcodes.insert(0x56, new_opcode("JUMP", 1, 0, 8));
    opcodes.insert(0x57, new_opcode("JUMPI", 2, 0, 10));
    opcodes.insert(0x58, new_opcode("PC", 0, 1, 2));
    opcodes.insert(0x59, new_opcode("MSIZE", 0, 1, 2));
    opcodes.insert(0x5a, new_opcode("GAS", 0, 1, 2));
    opcodes.insert(0x5b, new_opcode("JUMPDEST", 0, 0, 1));
    opcodes.insert(0x5c, new_opcode("TLOAD", 1, 1, 100));
    opcodes.insert(0x5d, new_opcode("TSTORE", 2, 0, 100));
    opcodes.insert(0x5e, new_opcode("MCOOPY", 3, 0, 3));

    // 5f, 60s & 70s: Push Operations
    opcodes.insert(0x5f, new_opcode("PUSH0", 0, 1, 2));
    for i in 1..33 {
        let name = format!("PUSH{}", i);
        opcodes.insert(0x5f + i, new_opcode(&name, 0, 1, 3));
    }

    // 80s: Duplication Operations
    for i in 1..17 {
      let name = format!("DUP{}", i);
      opcodes.insert(0x7f + i, new_opcode(&name, i as u32, i as u32 + 1, 3));
    // 90s: Exchange Operations
      let name = format!("SWAP{}", i);
      opcodes.insert(0x8f + i, new_opcode(&name, i as u32 + 1, i as u32 + 1, 3));
    }
    // closures
    opcodes.insert(0xf0, new_opcode("CREATE", 3, 1, 32000));
    opcodes.insert(0xf1, new_opcode("CALL", 7, 1, 100));
    opcodes.insert(0xf2, new_opcode("CALLCODE", 7, 1, 100));
    opcodes.insert(0xf3, new_opcode("RETURN", 2, 0, 0));
    opcodes.insert(0xf4, new_opcode("DELEGATECALL", 6, 0, 100));
    opcodes.insert(0xff, new_opcode("SELFDESCTRUCT", 1, 0, 5000));


    // a0s: Logging Operations
    opcodes.insert(0xa0, new_opcode("LOG0", 2, 0, 375));
    opcodes.insert(0xa1, new_opcode("LOG1", 3, 0, 750));
    opcodes.insert(0xa2, new_opcode("LOG2", 4, 0, 1125));
    opcodes.insert(0xa3, new_opcode("LOG3", 5, 0, 1500));
    opcodes.insert(0xa4, new_opcode("LOG4", 6, 0, 1875));

    opcodes
}

impl Stack {

}
