use std::array;

use serde::{Deserialize, Serialize};

use crate::runtime::Instruction;
use crate::runtime::LookupIdSampler;
use crate::runtime::MemoryRecordEnum;
use crate::runtime::Opcode;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LookupIds {
    AluLookupId(u128),
    SyscallLookupId(u128),
    MemoryLookupIds([u128; 2]),
    BranchLookupIds([u128; 3]),
    JumpLookupIds([u128; 2]),
    AuipcLookupId(u128),

    /// Used for the case of value of no lookupids.
    DefaultLookupIds,
}

impl LookupIds {
    pub fn new(instr: Instruction, rng_sampler: &mut impl LookupIdSampler) -> Self {
        let num_lookup_ids = if instr.is_alu_instruction()
            || instr.is_ecall_instruction()
            || instr.opcode == Opcode::AUIPC
        {
            1
        } else if instr.is_branch_instruction() {
            3
        } else if instr.is_jump_instruction() || instr.is_memory_instruction() {
            2
        } else {
            0
        };

        let lookup_ids = rng_sampler.sample(num_lookup_ids);

        if instr.is_alu_instruction() {
            LookupIds::AluLookupId(lookup_ids[0])
        } else if instr.is_ecall_instruction() {
            LookupIds::SyscallLookupId(lookup_ids[0])
        } else if instr.is_memory_instruction() {
            LookupIds::MemoryLookupIds([lookup_ids[0], lookup_ids[1]])
        } else if instr.is_branch_instruction() {
            LookupIds::BranchLookupIds([lookup_ids[0], lookup_ids[1], lookup_ids[2]])
        } else if instr.is_jump_instruction() {
            LookupIds::JumpLookupIds([lookup_ids[0], lookup_ids[1]])
        } else if instr.opcode == Opcode::AUIPC {
            LookupIds::AuipcLookupId(lookup_ids[0])
        } else {
            LookupIds::DefaultLookupIds
        }
    }

    pub fn new_sublookups(rng_sampler: &mut impl LookupIdSampler) -> [u128; 6] {
        let lookup_ids = rng_sampler.sample(6);
        array::from_fn(|i| lookup_ids[i])
    }
}

/// A standard format for describing CPU operations that need to be proven.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpuEvent {
    /// The current shard.
    pub shard: u32,

    /// The current channel.
    pub channel: u8,

    /// The current clock.
    pub clk: u32,

    /// The current program counter.
    pub pc: u32,

    /// The value of the next instruction's program counter. This value needs to be made public for
    /// the last row of each shard.
    pub next_pc: u32,

    /// The current instruction.
    pub instruction: Instruction,

    /// The first operand.
    pub a: u32,

    /// The memory access record for the first operand.
    pub a_record: Option<MemoryRecordEnum>,

    /// The second operand.
    pub b: u32,

    /// The memory access record for the second operand.
    pub b_record: Option<MemoryRecordEnum>,

    /// The third operand.
    pub c: u32,

    /// The memory access record for the third operand.
    pub c_record: Option<MemoryRecordEnum>,

    /// The memory value we potentially may access.
    pub memory: Option<u32>,

    /// The memory access record for the memory value.
    pub memory_record: Option<MemoryRecordEnum>,

    /// Exit code called with halt.
    pub exit_code: u32,

    pub lookup_ids: LookupIds,
}
