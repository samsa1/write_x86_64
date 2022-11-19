/// Various Symbols for dwarf
/// [https://dwarfstd.org/doc/DWARF4.pdf]
pub enum Symbols {
    /// Push lit on the stack
    /// Valid only for integers 0 <= .. <= 31
    DW_OP_lit(u8),

    /// Push an address on the stack
    DW_OP_addr(()),

    /// Push a 1-byte unsigned constant on the stack
    DW_OP_const1u(u8),
    /// Push a 2-byte unsigned constant on the stack
    DW_OP_const2u(u16),
    /// Push a 4-byte unsigned constant on the stack
    DW_OP_const4u(u32),
    /// Push a 8-byte unsigned constant on the stack
    DW_OP_const8u(u64),
    /// Push a little endian 16-byte unsigned constant on the stack
    DW_OP_constu(u128),

    /// Push a 1-byte signed constant on the stack
    DW_OP_const1s(i8),
    /// Push a 2-byte signed constant on the stack
    DW_OP_const2s(i16),
    /// Push a 4-byte signed constant on the stack
    DW_OP_const4s(i32),
    /// Push a 8-byte signed constant on the stack
    DW_OP_const8s(i64),
    /// Push a little 16-byte signed constant on the stack
    DW_OP_consts(i128),

    /// Duplicate value on the stack
    DW_OP_dup,
    /// Pops value from the stack
    DW_OP_drop,
    /// Pick value in the stack and push it on top
    DW_OP_pick(u8),
    /// Equivalent to DW_OP_pick(1)
    DW_OP_over,
    /// Swaps to 2 entries
    DW_OP_swap,
    /// Rotate three first entries
    DW_OP_rot,
    /// Changes the value on the stack by the value it points to
    DW_OP_deref,
    /// DW_OP_deref but with size specified
    DW_OP_deref_size(u8),
    /// See documentation
    DW_OP_xderef,
    /// See documentation
    DW_OP_xderef_size(u8),
    /// See documentation
    DW_OP_push_object_address,
    /// See documentation
    DW_OP_form_tls_address,
    /// See documentation
    DW_OP_call_frame_cfa,

    /// Abs on the top value (poped than pushed)
    DW_OP_abs,
    /// And on the 2 top values (poped than pushed)
    DW_OP_and,
    /// Div on the 2 top values (poped than pushed)
    DW_OP_div,
    /// Minus on the 2 top values (poped than pushed)
    DW_OP_minus,
    /// Mod on the 2 top values (poped than pushed)
    DW_OP_mod,
    /// Mul on the 2 top values (poped than pushed)
    DW_OP_mul,
    /// Neg on the top value (poped than pushed)
    DW_OP_neg,
    /// Not on the top value (poped than pushed)
    DW_OP_not,
    /// Or on the 2 top values (poped than pushed)
    DW_OP_or,
    /// Plus on the 2 top values (poped than pushed)
    DW_OP_plus,
    /// Add on the top value and constant (poped than pushed)
    DW_OP_plus_uconst(u128),
    /// Shl on the 2 top values (poped than pushed)
    DW_OP_shl,
    /// Shr on the 2 top values (poped than pushed)
    DW_OP_shr,
    /// Shra on the 2 top values (poped than pushed)
    DW_OP_shra,
    /// Xor on the 2 top values (poped than pushed)
    DW_OP_xor,

    /// Le on the 2 top values (poped than pushed)
    DW_OP_le,
    /// Ge on the 2 top values (poped than pushed)
    DW_OP_ge,
    /// Eq on the 2 top values (poped than pushed)
    DW_OP_eq,
    /// Lt on the 2 top values (poped than pushed)
    DW_OP_lt,
    /// Gt on the 2 top values (poped than pushed)
    DW_OP_gt,
    /// Ne on the 2 top values (poped than pushed)
    DW_OP_ne,

    /// See documentation
    DW_OP_skip(i16),
    /// See documentation
    DW_OP_bra(i16),
    /// See documentation
    DW_OP_call2(u16),
    /// See documentation
    DW_OP_call4(u16),

    /// Placeholder
    DW_OP_nop,
}
