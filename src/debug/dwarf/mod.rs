///! Section will be missing lost of documentation
///! See [https://dwarfstd.org/doc/DWARF4.pdf] for more informations
use std::collections::HashMap;

use crate::data::*;
use crate::reg::Label;
use crate::{Data, Segment};

pub mod consts;

struct Debug {
    abbrev: Data,
    info: Data,
}

impl Debug {
    fn empty() -> Self {
        Self {
            abbrev: Data::empty(),
            info: Data::empty(),
        }
    }

    fn concat(&mut self, other: Self) {
        self.abbrev += other.abbrev;
        self.info += other.info;
    }
}

pub struct Context {
    tmp_id: usize,
    total_length: usize,
    hashmap: HashMap<String, usize>,
    loc_start: Option<Label>,
    line_start: Option<Label>,
    macinfo_start: Option<Label>,
    ranges_start: Option<Label>,
    prefix: String,
}

impl Context {
    /// Creates a new structure with a reserved prefix for labels
    /// You need to ensure that no other label will start with the same prefix
    pub fn new(prefix: String) -> Self {
        Self {
            tmp_id: 0,
            total_length: 0,
            hashmap: HashMap::new(),
            loc_start: None,
            line_start: None,
            macinfo_start: None,
            ranges_start: None,
            prefix,
        }
    }

    fn get_offset(&mut self, str: String) -> usize {
        match self.hashmap.get(&str) {
            Some(pos) => *pos,
            None => {
                let out = self.total_length;
                self.total_length += str.len() + 1;
                self.hashmap.insert(str, out);
                out
            }
        }
    }

    fn to_data(self) -> Data {
        let mut v: Vec<_> = self.hashmap.into_iter().collect();
        v.sort_by(|a, b| a.1.cmp(&b.1));
        let mut data = Data::empty();
        let mut i = 0;
        for (el, i2) in v {
            assert_eq!(i, i2);
            i += el.len() + 1;
            data += dasciz(el)
        }
        data
    }

    fn new_tmp(&mut self) -> Label {
        let id = self.tmp_id;
        self.tmp_id += 1;
        Label::from_str(format!("{}_tmp_{}", self.prefix, id))
    }

    pub fn set_loc_start(&mut self, lab: Label) {
        self.loc_start = Some(lab)
    }

    pub fn get_loc_start(&self) -> &Option<Label> {
        &self.loc_start
    }

    pub fn set_line_start(&mut self, lab: Label) {
        self.line_start = Some(lab)
    }

    pub fn get_line_start(&self) -> &Option<Label> {
        &self.line_start
    }

    pub fn set_macinfo_start(&mut self, lab: Label) {
        self.macinfo_start = Some(lab)
    }

    pub fn get_macinfo_start(&self) -> &Option<Label> {
        &self.macinfo_start
    }

    pub fn set_ranges_start(&mut self, lab: Label) {
        self.ranges_start = Some(lab)
    }

    pub fn get_ranges_start(&self) -> &Option<Label> {
        &self.ranges_start
    }
}

#[macro_use]
mod macros;

trait DebugInfo: Default {
    fn to_data(self, context: &mut Context) -> Debug;
}

/*
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
}*/

#[derive(Default)]
pub struct AccessDeclaration {
    pub accessibility: Option<consts::dw_access::DWAccess>,
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
}

impl DebugInfo for AccessDeclaration {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        info1_data!(debug, self, accessibility);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct ArrayType {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    allocated: (),
    associated: (),
    pub bit_size: Option<u32>,
    bit_stride: (),
    pub byte_size: Option<u32>,
    data_location: (),
    declaration: bool,
    pub description: Option<String>,
    pub name: Option<String>,
    pub ordering: Option<consts::dw_ord::Ordering>,
    sibling: (),
    specification: (),
    pub start_scope: Option<Label>,
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for ArrayType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, allocated);
        not_done!(self, associated);
        data4!(debug, self, bit_size);
        not_done!(self, bit_stride);
        data4!(debug, self, byte_size);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        info1_data!(debug, self, ordering);
        not_done!(self, sibling);
        not_done!(self, specification);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct BaseType {
    allocated: (),
    associated: (),
    pub binary_scale: Option<i32>,
    bit_offset: (),
    pub bit_size: Option<u32>,
    pub byte_size: Option<u32>,
    pub data_bit_offset: Option<u32>,
    data_location: (),
    pub decimal_scale: Option<i32>,
    pub decimal_sign: Option<consts::dw_ds::DecSign>,
    pub description: Option<String>,
    pub digit_count: Option<u32>,
    pub encoding: Option<consts::dw_ate::BaseType>,
    pub endianity: Option<consts::dw_end::Endianity>,
    pub name: Option<String>,
    pub picture_string: Option<String>,
    sibling: (),
    small: (),
}

impl DebugInfo for BaseType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, allocated);
        not_done!(self, associated);
        sdata4!(debug, self, binary_scale);
        not_done!(self, bit_offset);
        data4!(debug, self, bit_size);
        data4!(debug, self, byte_size);
        data4!(debug, self, data_bit_offset);
        not_done!(self, data_location);
        sdata4!(debug, self, decimal_scale);
        info1_data!(debug, self, decimal_sign);
        string_data!(debug, self, description, context);
        data4!(debug, self, digit_count);
        info1_data!(debug, self, encoding);
        info1_data!(debug, self, endianity);
        string_data!(debug, self, name, context);
        string_data!(debug, self, picture_string, context);
        not_done!(self, sibling);
        not_done!(self, small);
        end!(debug);
        debug
    }
}
#[derive(Default)]
pub struct CatchBlock {
    abstract_origin: (),
    pub low_high_pc: Option<(Label, Label)>,
    pub ranges: Option<Label>,
    pub segment: Option<Label>,
    sibling: (),
}

impl DebugInfo for CatchBlock {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        low_high_pc!(debug, self, context);
        rangelistptr!(debug, self, ranges, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct ClassType {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    allocated: (),
    associated: (),
    pub bit_size: Option<u32>,
    pub byte_size: Option<u32>,
    data_location: (),
    declaration: bool,
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    signature: (),
    specification: (),
    pub start_scope: Option<Label>,
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for ClassType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, allocated);
        not_done!(self, associated);
        data4!(debug, self, bit_size);
        data4!(debug, self, byte_size);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, signature);
        not_done!(self, specification);
        rangelistptr!(debug, self, start_scope, context);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct CommonBlock {
    pub declaration: bool,
    pub description: Option<String>,
    pub linkage_name: Option<String>,
    location: (),
    pub name: Option<String>,
    pub segment: Option<Label>,
    sibling: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for CommonBlock {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, linkage_name, context);
        not_done!(self, location);
        string_data!(debug, self, name, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct CommonInclusion {
    common_reference: (),
    pub declaration: bool,
    sibling: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for CommonInclusion {
    fn to_data(self, _context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, common_reference);
        flag_data!(debug, self, declaration);
        not_done!(self, sibling);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}
#[derive(Default)]
pub struct CompileUnit {
    base_types: (),
    pub ranges: Option<Label>,
    pub segment: Option<Label>,
    pub low_high_pc: Option<(Label, Label)>,
    pub name: Option<String>,
    pub language: Option<consts::dw_lang::DWLang>,
    stmt_list: (),
    pub macro_info: Option<consts::dw_macinfo::MacInfo>,
    pub comp_dir: Option<String>,
    pub producer: Option<String>,
    pub identifier_case: Option<consts::dw_id::DWId>,
    pub main_subprogram: bool,
    pub use_utf8: bool,
}

impl DebugInfo for CompileUnit {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, base_types);
        rangelistptr!(debug, self, ranges, context);
        loclist_ptr!(debug, self, segment, context);
        low_high_pc!(debug, self, context);
        string_data!(debug, self, name, context);
        info_data!(debug, self, language);
        not_done!(self, stmt_list);
        info1_data!(debug, self, macro_info);
        string_data!(debug, self, comp_dir, context);
        string_data!(debug, self, producer, context);
        info1_data!(debug, self, identifier_case);
        flag_data!(debug, self, main_subprogram);
        flag_data!(debug, self, use_utf8);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct Condition {
    pub name: Option<String>,
    sibling: (),
}

impl DebugInfo for Condition {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct ConstType {
    allocated: (),
    associated: (),
    data_location: (),
    pub name: Option<String>,
    sibling: (),
    _type: (),
}

impl DebugInfo for ConstType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct Constant {
    pub accessibility: Option<consts::dw_access::DWAccess>,
    const_value: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub endianity: Option<consts::dw_end::Endianity>,
    pub external: bool,
    pub linkage_name: Option<String>,
    pub name: Option<String>,
    sibling: (),
    pub start_scope: Option<Label>,
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for Constant {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        info1_data!(debug, self, accessibility);
        not_done!(self, const_value);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        info1_data!(debug, self, endianity);
        flag_data!(debug, self, external);
        string_data!(debug, self, linkage_name, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct DwarfProcedure {
    location: (),
}

impl DebugInfo for DwarfProcedure {
    fn to_data(self, _context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, location);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct EntryPoint {
    address_class: (),
    pub description: Option<String>,
    pub frame_base: Option<Label>,
    pub linkage_name: Option<String>,
    pub low_pc: Option<Label>,
    pub name: Option<String>,
    pub return_addr: Option<Label>,
    pub segment: Option<Label>,
    sibling: (),
    static_link: (),
    _type: (),
}

impl DebugInfo for EntryPoint {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, address_class);
        string_data!(debug, self, description, context);
        loclist_ptr!(debug, self, frame_base, context);
        string_data!(debug, self, linkage_name, context);
        addr_data!(debug, self, low_pc);
        string_data!(debug, self, name, context);
        loclist_ptr!(debug, self, return_addr, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        not_done!(self, static_link);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct EnumerationType {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    allocated: (),
    associated: (),
    pub bit_size: Option<u32>,
    bit_stride: (),
    pub byte_size: Option<u32>,
    byte_stride: (),
    data_location: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub enum_class: bool,
    pub name: Option<String>,
    sibling: (),
    signature: (),
    specification: (),
    pub start_scope: Option<Label>,
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for EnumerationType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, allocated);
        not_done!(self, associated);
        data4!(debug, self, bit_size);
        not_done!(self, bit_stride);
        data4!(debug, self, byte_size);
        not_done!(self, byte_stride);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        flag_data!(debug, self, enum_class);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, signature);
        not_done!(self, specification);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct Enumerator {
    const_value: (),
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
}

impl DebugInfo for Enumerator {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, const_value);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct FileType {
    abstract_origin: (),
    allocated: (),
    associated: (),
    pub bit_size: Option<u32>,
    pub byte_size: Option<u32>,
    data_location: (),
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    pub start_scope: Option<Label>,
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for FileType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        not_done!(self, allocated);
        not_done!(self, associated);
        data4!(debug, self, bit_size);
        data4!(debug, self, byte_size);
        not_done!(self, data_location);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct FormalParameter {
    abstract_origin: (),
    pub artificial: bool,
    const_value: (),
    default_value: (),
    pub description: Option<String>,
    pub endianity: Option<consts::dw_end::Endianity>,
    pub is_optional: bool,
    location: (),
    pub name: Option<String>,
    pub segment: Option<Label>,
    sibling: (),
    _type: (),
    pub variable_parameter: bool,
}

impl DebugInfo for FormalParameter {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        flag_data!(debug, self, artificial);
        not_done!(self, const_value);
        not_done!(self, default_value);
        string_data!(debug, self, description, context);
        info1_data!(debug, self, endianity);
        flag_data!(debug, self, is_optional);
        not_done!(self, location);
        string_data!(debug, self, name, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        flag_data!(debug, self, variable_parameter);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct Friend {
    abstract_origin: (),
    friend: (),
    sibling: (),
}

impl DebugInfo for Friend {
    fn to_data(self, _context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        not_done!(self, friend);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct ImportedDeclaration {
    pub accessibility: Option<consts::dw_access::DWAccess>,
    pub description: Option<String>,
    import: (),
    pub name: Option<String>,
    sibling: (),
    pub start_scope: Option<Label>,
}

impl DebugInfo for ImportedDeclaration {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        info1_data!(debug, self, accessibility);
        string_data!(debug, self, description, context);
        not_done!(self, import);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        rangelistptr!(debug, self, start_scope, context);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct ImportedModule {
    import: (),
    sibling: (),
    pub start_scope: Option<Label>,
}

impl DebugInfo for ImportedModule {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, import);
        not_done!(self, sibling);
        rangelistptr!(debug, self, start_scope, context);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct ImportedUnit {
    import: (),
}

impl DebugInfo for ImportedUnit {
    fn to_data(self, _context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, import);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct Inheritance {
    pub accessibility: Option<consts::dw_access::DWAccess>,
    pub data_member_location: Option<Label>,
    sibling: (),
    _type: (),
    pub virtuality: Option<consts::dw_virtuality::Virtuality>,
}

impl DebugInfo for Inheritance {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        info1_data!(debug, self, accessibility);
        loclist_ptr!(debug, self, data_member_location, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        info1_data!(debug, self, virtuality);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct InlinedSubroutine {
    abstract_origin: (),
    pub call_column: Option<u32>,
    pub call_file: Option<u32>,
    pub call_line: Option<u32>,
    pub const_expr: bool,
    pub entry_pc: Option<Label>,
    pub low_high_pc: Option<(Label, Label)>,
    pub ranges: Option<Label>,
    pub return_addr: Option<Label>,
    pub segment: Option<Label>,
    sibling: (),
    pub start_scope: Option<Label>,
    trampoline: (),
}

impl DebugInfo for InlinedSubroutine {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        data4!(debug, self, call_column);
        data4!(debug, self, call_file);
        data4!(debug, self, call_line);
        flag_data!(debug, self, const_expr);
        addr_data!(debug, self, entry_pc);
        low_high_pc!(debug, self, context);
        rangelistptr!(debug, self, ranges, context);
        loclist_ptr!(debug, self, return_addr, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, trampoline);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct InterfaceType {
    pub accessibility: Option<consts::dw_access::DWAccess>,
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    pub start_scope: Option<Label>,
}

impl DebugInfo for InterfaceType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        info1_data!(debug, self, accessibility);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        rangelistptr!(debug, self, start_scope, context);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct LabelTag {
    abstract_origin: (),
    pub description: Option<String>,
    pub low_pc: Option<Label>,
    pub name: Option<String>,
    pub segment: Option<Label>,
    pub start_scope: Option<Label>,
    sibling: (),
}

impl DebugInfo for LabelTag {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        string_data!(debug, self, description, context);
        addr_data!(debug, self, low_pc);
        string_data!(debug, self, name, context);
        loclist_ptr!(debug, self, segment, context);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct LexicalBlock {
    abstract_origin: (),
    pub description: Option<String>,
    pub low_high_pc: Option<(Label, Label)>,
    pub name: Option<String>,
    pub ranges: Option<Label>,
    pub segment: Option<Label>,
    sibling: (),
}

impl DebugInfo for LexicalBlock {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        string_data!(debug, self, description, context);
        low_high_pc!(debug, self, context);
        string_data!(debug, self, name, context);
        rangelistptr!(debug, self, ranges, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct Member {
    accessibility: Option<consts::dw_access::DWAccess>,
    bit_offset: (),
    pub bit_size: Option<u32>,
    pub data_bit_offset: Option<u32>,
    pub data_member_location: Option<Label>,
    pub declaration: bool,
    pub description: Option<String>,
    pub mutable: bool,
    pub name: Option<String>,
    sibling: (),
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for Member {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        info1_data!(debug, self, accessibility);
        not_done!(self, bit_offset);
        data4!(debug, self, bit_size);
        data4!(debug, self, data_bit_offset);
        loclist_ptr!(debug, self, data_member_location, context);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        flag_data!(debug, self, mutable);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct Module {
    pub accessibility: Option<consts::dw_access::DWAccess>,
    pub declaration: bool,
    pub description: Option<String>,
    pub entry_pc: Option<Label>,
    pub low_high_pc: Option<(Label, Label)>,
    pub name: Option<String>,
    priority: (),
    pub ranges: Option<Label>,
    pub segment: Option<Label>,
    sibling: (),
    specification: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for Module {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        info1_data!(debug, self, accessibility);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        addr_data!(debug, self, entry_pc);
        low_high_pc!(debug, self, context);
        string_data!(debug, self, name, context);
        not_done!(self, priority);
        rangelistptr!(debug, self, ranges, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        not_done!(self, specification);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct NameList {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    pub declaration: bool,
    pub name: Option<String>,
    sibling: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for NameList {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct NameListItem {
    namelist_item: (),
    sibling: (),
}

impl DebugInfo for NameListItem {
    fn to_data(self, _context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, namelist_item);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct NameSpace {
    pub description: Option<String>,
    extension: (),
    pub name: Option<String>,
    sibling: (),
    pub start_scope: Option<Label>,
}

impl DebugInfo for NameSpace {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        string_data!(debug, self, description, context);
        not_done!(self, extension);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        rangelistptr!(debug, self, start_scope, context);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct PackedType {
    allocated: (),
    associated: (),
    data_location: (),
    pub name: Option<String>,
    sibling: (),
    _type: (),
}

impl DebugInfo for PackedType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct PartialUnit {
    base_types: (),
    pub comp_dir: Option<String>,
    pub description: Option<String>,
    pub identifier_case: Option<consts::dw_id::DWId>,
    pub language: Option<consts::dw_lang::DWLang>,
    pub low_high_pc: Option<(Label, Label)>,
    pub macro_info: Option<consts::dw_macinfo::MacInfo>,
    pub main_subprogram: bool,
    pub name: Option<String>,
    pub producer: Option<String>,
    pub ranges: Option<Label>,
    pub segment: Option<Label>,
    stmt_list: (),
    pub use_utf8: bool,
}

impl DebugInfo for PartialUnit {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, base_types);
        string_data!(debug, self, comp_dir, context);
        string_data!(debug, self, description, context);
        info1_data!(debug, self, identifier_case);
        info_data!(debug, self, language);
        low_high_pc!(debug, self, context);
        info1_data!(debug, self, macro_info);
        flag_data!(debug, self, main_subprogram);
        string_data!(debug, self, name, context);
        string_data!(debug, self, producer, context);
        rangelistptr!(debug, self, ranges, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, stmt_list);
        flag_data!(debug, self, use_utf8);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct PointerType {
    address_class: (),
    allocated: (),
    associated: (),
    data_location: (),
    pub name: Option<String>,
    sibling: (),
    _type: (),
}

impl DebugInfo for PointerType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, address_class);
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct PtrToMemberType {
    abstract_origin: (),
    address_class: (),
    allocated: (),
    associated: (),
    containing_type: (),
    data_location: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    _type: (),
    pub use_location: Option<Label>,
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for PtrToMemberType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        not_done!(self, address_class);
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, containing_type);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        loclist_ptr!(debug, self, use_location, context);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct ReferenceType {
    address_class: (),
    allocated: (),
    associated: (),
    data_location: (),
    pub name: Option<String>,
    sibling: (),
    _type: (),
}

impl DebugInfo for ReferenceType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, address_class);
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct RestrictType {
    allocated: (),
    associated: (),
    data_location: (),
    pub name: Option<String>,
    sibling: (),
    _type: (),
}

impl DebugInfo for RestrictType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct RValueReferenceType {
    address_class: (),
    allocated: (),
    associated: (),
    data_location: (),
    pub name: Option<String>,
    sibling: (),
    _type: (),
}

impl DebugInfo for RValueReferenceType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, address_class);
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct SetType {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    allocated: (),
    associated: (),
    pub bit_size: Option<u32>,
    pub byte_size: Option<u32>,
    data_location: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub name: Option<String>,
    pub start_scope: Option<Label>,
    sibling: (),
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for SetType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, allocated);
        not_done!(self, associated);
        data4!(debug, self, bit_size);
        data4!(debug, self, byte_size);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct SharedType {
    allocated: (),
    associated: (),
    count: (),
    data_location: (),
    pub name: Option<String>,
    sibling: (),
    _type: (),
}

impl DebugInfo for SharedType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, count);
        not_done!(self, data_location);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct StringType {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    allocated: (),
    associated: (),
    pub bit_size: Option<u32>,
    pub byte_size: Option<u32>,
    data_location: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    pub start_scope: Option<Label>,
    string_length: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for StringType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, allocated);
        not_done!(self, associated);
        data4!(debug, self, bit_size);
        data4!(debug, self, byte_size);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, string_length);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct StructureType {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    allocated: (),
    associated: (),
    pub bit_size: Option<u32>,
    pub byte_size: Option<u32>,
    data_location: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    signature: (),
    specification: (),
    pub start_scope: Option<Label>,
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for StructureType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, allocated);
        not_done!(self, associated);
        data4!(debug, self, bit_size);
        data4!(debug, self, byte_size);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, signature);
        not_done!(self, specification);
        rangelistptr!(debug, self, start_scope, context);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct SubProgram {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    address_class: (),
    articifial: (),
    pub calling_convention: Option<consts::dw_cc::DWCC>,
    pub declaration: bool,
    pub description: Option<String>,
    pub elemental: bool,
    pub entry_pc: Option<Label>,
    pub explicit: bool,
    pub external: bool,
    pub frame_base: Option<Label>,
    pub inline: Option<consts::dw_inl::Inline>,
    pub linkage_name: Option<String>,
    pub low_high_pc: Option<(Label, Label)>,
    pub main_subprogram: bool,
    pub name: Option<String>,
    object_pointer: (),
    pub prototyped: bool,
    pub pure: bool,
    pub ranges: Option<Label>,
    pub recursive: bool,
    pub return_addr: Option<Label>,
    pub segment: Option<Label>,
    sibling: (),
    specification: (),
    pub start_scope: Option<Label>,
    static_link: (),
    trampoline: (),
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
    pub virtuality: Option<consts::dw_virtuality::Virtuality>,
    pub vtable_elem_location: Option<Label>,
}

impl DebugInfo for SubProgram {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, address_class);
        not_done!(self, articifial);
        info1_data!(debug, self, calling_convention);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        flag_data!(debug, self, elemental);
        addr_data!(debug, self, entry_pc);
        flag_data!(debug, self, explicit);
        flag_data!(debug, self, external);
        loclist_ptr!(debug, self, frame_base, context);
        info1_data!(debug, self, inline);
        string_data!(debug, self, linkage_name, context);
        low_high_pc!(debug, self, context);
        flag_data!(debug, self, main_subprogram);
        string_data!(debug, self, name, context);
        not_done!(self, object_pointer);
        flag_data!(debug, self, prototyped);
        flag_data!(debug, self, pure);
        rangelistptr!(debug, self, ranges, context);
        flag_data!(debug, self, recursive);
        loclist_ptr!(debug, self, return_addr, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        not_done!(self, specification);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, static_link);
        not_done!(self, trampoline);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        info1_data!(debug, self, virtuality);
        loclist_ptr!(debug, self, vtable_elem_location, context);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct SubrangeType {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    allocated: (),
    associated: (),
    pub bit_size: Option<u32>,
    bit_stride: (),
    pub byte_size: Option<u32>,
    byte_stride: (),
    count: (),
    data_location: (),
    pub declaration: bool,
    pub description: Option<String>,
    lower_bound: (),
    pub name: Option<String>,
    sibling: (),
    pub threads_scaled: bool,
    _type: (),
    upper_bound: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for SubrangeType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, allocated);
        not_done!(self, associated);
        data4!(debug, self, bit_size);
        not_done!(self, bit_stride);
        data4!(debug, self, byte_size);
        not_done!(self, byte_stride);
        not_done!(self, count);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        not_done!(self, lower_bound);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        flag_data!(debug, self, threads_scaled);
        not_done!(self, _type);
        not_done!(self, upper_bound);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct SubroutineType {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    address_class: (),
    allocated: (),
    associated: (),
    data_location: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub name: Option<String>,
    pub prototyped: bool,
    sibling: (),
    pub start_scope: Option<Label>,
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for SubroutineType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, address_class);
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        flag_data!(debug, self, prototyped);
        not_done!(self, sibling);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct TemplateAlias {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    allocated: (),
    associated: (),
    data_location: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    signature: (),
    pub start_scope: Option<Label>,
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for TemplateAlias {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, signature);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct TemplateTypeParameter {
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    _type: (),
}

impl DebugInfo for TemplateTypeParameter {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct TemplateValueParameter {
    const_value: (),
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    _type: (),
}

impl DebugInfo for TemplateValueParameter {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, const_value);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct ThrownType {
    allocated: (),
    associated: (),
    data_location: (),
    sibling: (),
    _type: (),
}

impl DebugInfo for ThrownType {
    fn to_data(self, _context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct TryBlock {
    abstract_origin: (),
    pub low_high_pc: Option<(Label, Label)>,
    pub ranges: Option<Label>,
    pub segment: Option<Label>,
    sibling: (),
}

impl DebugInfo for TryBlock {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        low_high_pc!(debug, self, context);
        rangelistptr!(debug, self, ranges, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct Typedef {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    allocated: (),
    associated: (),
    data_location: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    pub start_scope: Option<Label>,
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for Typedef {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct TypeUnit {
    pub language: Option<consts::dw_lang::DWLang>,
}

impl DebugInfo for TypeUnit {
    fn to_data(self, _context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        info_data!(debug, self, language);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct UnionType {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    allocated: (),
    associated: (),
    pub bit_size: Option<u32>,
    pub byte_size: Option<u32>,
    data_location: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub name: Option<String>,
    sibling: (),
    signature: (),
    specification: (),
    pub start_scope: Option<Label>,
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for UnionType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        not_done!(self, allocated);
        not_done!(self, associated);
        data4!(debug, self, bit_size);
        data4!(debug, self, byte_size);
        not_done!(self, data_location);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, signature);
        not_done!(self, specification);
        rangelistptr!(debug, self, start_scope, context);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct UnspecifiedParameters {
    abstract_origin: (),
    pub artificial: bool,
    sibling: (),
}

impl DebugInfo for UnspecifiedParameters {
    fn to_data(self, _context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        flag_data!(debug, self, artificial);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct UnspecifiedType {
    pub description: Option<String>,
    pub name: Option<String>,
}

impl DebugInfo for UnspecifiedType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        string_data!(debug, self, description, context);
        string_data!(debug, self, name, context);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct Variable {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    pub const_expr: bool,
    const_value: (),
    pub declaration: bool,
    pub description: Option<String>,
    pub endianity: Option<consts::dw_end::Endianity>,
    pub external: bool,
    pub linkage_name: Option<String>,
    location: (),
    pub name: Option<String>,
    pub segment: Option<Label>,
    sibling: (),
    specification: (),
    pub start_scope: Option<Label>,
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for Variable {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        flag_data!(debug, self, const_expr);
        not_done!(self, const_value);
        flag_data!(debug, self, declaration);
        string_data!(debug, self, description, context);
        info1_data!(debug, self, endianity);
        flag_data!(debug, self, external);
        string_data!(debug, self, linkage_name, context);
        not_done!(self, location);
        string_data!(debug, self, name, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        not_done!(self, specification);
        rangelistptr!(debug, self, start_scope, context);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct Variant {
    pub accessibility: Option<consts::dw_access::DWAccess>,
    abstract_origin: (),
    pub declaration: bool,
    discr_list: (),
    pub discr_value: Option<consts::dw_dsc::Discriminant>,
    sibling: (),
}

impl DebugInfo for Variant {
    fn to_data(self, _context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        info1_data!(debug, self, accessibility);
        not_done!(self, abstract_origin);
        flag_data!(debug, self, declaration);
        not_done!(self, discr_list);
        info1_data!(debug, self, discr_value);
        not_done!(self, sibling);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct VariantPart {
    abstract_origin: (),
    pub accessibility: Option<consts::dw_access::DWAccess>,
    pub declaration: bool,
    discr: (),
    sibling: (),
    _type: (),
}

impl DebugInfo for VariantPart {
    fn to_data(self, _context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, abstract_origin);
        info1_data!(debug, self, accessibility);
        flag_data!(debug, self, declaration);
        not_done!(self, discr);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct VolatileType {
    allocated: (),
    associated: (),
    data_location: (),
    pub name: Option<String>,
    sibling: (),
    _type: (),
}

impl DebugInfo for VolatileType {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        not_done!(self, allocated);
        not_done!(self, associated);
        not_done!(self, data_location);
        string_data!(debug, self, name, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        end!(debug);
        debug
    }
}

#[derive(Default)]
pub struct WithStmt {
    pub accessibility: Option<consts::dw_access::DWAccess>,
    address_class: (),
    pub declaration: bool,
    location: (),
    pub low_high_pc: Option<(Label, Label)>,
    pub ranges: Option<Label>,
    pub segment: Option<Label>,
    sibling: (),
    _type: (),
    pub visibility: Option<consts::dw_vis::Visibility>,
}

impl DebugInfo for WithStmt {
    fn to_data(self, context: &mut Context) -> Debug {
        let mut debug = Debug::empty();
        info1_data!(debug, self, accessibility);
        not_done!(self, address_class);
        flag_data!(debug, self, declaration);
        not_done!(self, location);
        low_high_pc!(debug, self, context);
        rangelistptr!(debug, self, ranges, context);
        loclist_ptr!(debug, self, segment, context);
        not_done!(self, sibling);
        not_done!(self, _type);
        info1_data!(debug, self, visibility);
        end!(debug);
        debug
    }
}

/// Structure to represent a DIE
pub enum DIE {
    DwTagAccessDeclaration(AccessDeclaration),
    DwTagArrayType(ArrayType),
    DwTagBaseType(BaseType),
    DwTagCatchBlock(CatchBlock),
    DwTagClassType(ClassType),
    DwTagCommonBlock(CommonBlock),
    DwTagCommonInclusion(CommonInclusion),
    DwTagCompileUnit(CompileUnit),
    DwTagCondition(Condition),
    DwTagConstType(ConstType),
    DwTagConstant(Constant),
    DwTagDwarfProcedure(DwarfProcedure),
    DwTagEntryPoint(EntryPoint),
    DwTagEnumerationType(EnumerationType),
    DwTagEnumerator(Enumerator),
    DwTagFileType(FileType),
    DwTagFormalParameter(FormalParameter),
    DwTagFriend(Friend),
    DwTagImportedDeclaration(ImportedDeclaration),
    DwTagImportedModule(ImportedModule),
    DwTagImportedUnit(ImportedUnit),
    DwTagInheritance(Inheritance),
    DwTagInlinedSubroutine(InlinedSubroutine),
    DwTagInterfaceType(InterfaceType),
    DwTagLabel(LabelTag),
    DwTagLexicalBlock(LexicalBlock),
    DwTagMember(Member),
    DwTagModule(Module),
    DwTagNamelist(NameList),
    DwTagNamelistItem(NameListItem),
    DwTagNamespace(NameSpace),
    DwTagPackedType(PackedType),
    DwTagPartialUnit(PartialUnit),
    DwTagPointerType(PointerType),
    DwTagPtrToMemberType(PtrToMemberType),
    DwTagReferenceType(ReferenceType),
    DwTagRestrictType(ReferenceType),
    DwTagRvalueReferenceType(RValueReferenceType),
    DwTagSetType(SetType),
    DwTagSharedType(SharedType),
    DwTagStringType(StringType),
    DwTagStructureType(StructureType),
    DwTagSubprogram(SubProgram),
    DwTagSubrangeType(SubrangeType),
    DwTagSubroutineType(SubroutineType),
    DwTagTemplateAlias(TemplateAlias),
    DwTagTemplateTypeParameter(TemplateTypeParameter),
    DwTagTemplateValueParameter(TemplateValueParameter),
    DwTagThrownType(ThrownType),
    DwTagTryBlock(TryBlock),
    DwTagTypedef(Typedef),
    DwTagTypeUnit(TypeUnit),
    DwTagUnionType(UnionType),
    DwTagUnspecifiedParameters(UnspecifiedParameters),
    DwTagUnspecifiedType(UnspecifiedType),
    DwTagVariable(Variable),
    DwTagVariant(Variant),
    DwTagVariantPart(VariantPart),
    DwTagVolatileType(VolatileType),
    DwTagWithStmt(WithStmt),
}

impl DIE {
    fn id(&self) -> u8 {
        match self {
            Self::DwTagAccessDeclaration(_) => 0x23,
            Self::DwTagArrayType(_) => 0x01,
            Self::DwTagBaseType(_) => 0x24,
            Self::DwTagCatchBlock(_) => 0x25,
            Self::DwTagClassType(_) => 0x02,
            Self::DwTagCommonBlock(_) => 0x1a,
            Self::DwTagCommonInclusion(_) => 0x1b,
            Self::DwTagCompileUnit(_) => 0x11,
            Self::DwTagCondition(_) => 0x3f,
            Self::DwTagConstType(_) => 0x26,
            Self::DwTagConstant(_) => 0x27,
            Self::DwTagDwarfProcedure(_) => 0x36,
            Self::DwTagEntryPoint(_) => 0x03,
            Self::DwTagEnumerationType(_) => 0x04,
            Self::DwTagEnumerator(_) => 0x28,
            Self::DwTagFileType(_) => 0x29,
            Self::DwTagFormalParameter(_) => 0x05,
            Self::DwTagFriend(_) => 0x2a,
            Self::DwTagImportedDeclaration(_) => 0x08,
            Self::DwTagImportedModule(_) => 0x3a,
            Self::DwTagImportedUnit(_) => 0x3d,
            Self::DwTagInheritance(_) => 0x1c,
            Self::DwTagInlinedSubroutine(_) => 0x1d,
            Self::DwTagInterfaceType(_) => 0x38,
            Self::DwTagLabel(_) => 0x0a,
            Self::DwTagLexicalBlock(_) => 0x0b,
            Self::DwTagMember(_) => 0x0d,
            Self::DwTagModule(_) => 0x1e,
            Self::DwTagNamelist(_) => 0x2b,
            Self::DwTagNamelistItem(_) => 0x2c,
            Self::DwTagNamespace(_) => 0x39,
            Self::DwTagPackedType(_) => 0x2d,
            Self::DwTagPartialUnit(_) => 0x3c,
            Self::DwTagPointerType(_) => 0x0f,
            Self::DwTagPtrToMemberType(_) => 0x1f,
            Self::DwTagReferenceType(_) => 0x10,
            Self::DwTagRestrictType(_) => 0x37,
            Self::DwTagRvalueReferenceType(_) => 0x42,
            Self::DwTagSetType(_) => 0x20,
            Self::DwTagSharedType(_) => 0x40,
            Self::DwTagStringType(_) => 0x12,
            Self::DwTagStructureType(_) => 0x13,
            Self::DwTagSubprogram(_) => 0x2e,
            Self::DwTagSubrangeType(_) => 0x21,
            Self::DwTagSubroutineType(_) => 0x15,
            Self::DwTagTemplateAlias(_) => 0x43,
            Self::DwTagTemplateTypeParameter(_) => 0x2f,
            Self::DwTagTemplateValueParameter(_) => 0x30,
            Self::DwTagThrownType(_) => 0x31,
            Self::DwTagTryBlock(_) => 0x32,
            Self::DwTagTypedef(_) => 0x16,
            Self::DwTagTypeUnit(_) => 0x41,
            Self::DwTagUnionType(_) => 0x17,
            Self::DwTagUnspecifiedParameters(_) => 0x18,
            Self::DwTagUnspecifiedType(_) => 0x3b,
            Self::DwTagVariable(_) => 0x34,
            Self::DwTagVariant(_) => 0x19,
            Self::DwTagVariantPart(_) => 0x33,
            Self::DwTagVolatileType(_) => 0x35,
            Self::DwTagWithStmt(_) => 0x22,
        }
    }
    fn tag_name(&self) -> &'static str {
        match self {
            DIE::DwTagAccessDeclaration(_) => "DW_TAG_access_declaration",
            DIE::DwTagArrayType(_) => "DW_TAG_array_type",
            DIE::DwTagBaseType(_) => "DW_TAG_base_type",
            DIE::DwTagCatchBlock(_) => "DW_TAG_catch_block",
            DIE::DwTagClassType(_) => "DW_TAG_class_type",
            DIE::DwTagCommonBlock(_) => "DW_TAG_common_block",
            DIE::DwTagCommonInclusion(_) => "DW_TAG_common_inclusion",
            DIE::DwTagCompileUnit(_) => "DW_TAG_compile_unit",
            DIE::DwTagCondition(_) => "DW_TAG_condition",
            DIE::DwTagConstType(_) => "DW_TAG_const_type",
            DIE::DwTagConstant(_) => "DW_TAG_constant",
            DIE::DwTagDwarfProcedure(_) => "DW_TAG_dwarf_procedure",
            DIE::DwTagEntryPoint(_) => "DW_TAG_entry_point",
            DIE::DwTagEnumerationType(_) => "DW_TAG_enumeration_type",
            DIE::DwTagEnumerator(_) => "DW_TAG_enumerator",
            DIE::DwTagFileType(_) => "DW_TAG_file_type",
            DIE::DwTagFormalParameter(_) => "DW_TAG_formal_parameter",
            DIE::DwTagFriend(_) => "DW_TAG_friend",
            DIE::DwTagImportedDeclaration(_) => "DW_TAG_imported_declaration",
            DIE::DwTagImportedModule(_) => "DW_TAG_imported_module",
            DIE::DwTagImportedUnit(_) => "DW_TAG_imported_unit",
            DIE::DwTagInheritance(_) => "DW_TAG_inheritance",
            DIE::DwTagInlinedSubroutine(_) => "DW_TAG_inlined_subroutine",
            DIE::DwTagInterfaceType(_) => "DW_TAG_interface_type",
            DIE::DwTagLabel(_) => "DW_TAG_label",
            DIE::DwTagLexicalBlock(_) => "DW_TAG_lexical_block",
            DIE::DwTagMember(_) => "DW_TAG_member",
            DIE::DwTagModule(_) => "DW_TAG_module",
            DIE::DwTagNamelist(_) => "DW_TAG_namelist",
            DIE::DwTagNamelistItem(_) => "DW_TAG_namelist_item",
            DIE::DwTagNamespace(_) => "DW_TAG_namespace",
            DIE::DwTagPackedType(_) => "DW_TAG_packed_type",
            DIE::DwTagPartialUnit(_) => "DW_TAG_partial_unit",
            DIE::DwTagPointerType(_) => "DW_TAG_pointer_type",
            DIE::DwTagPtrToMemberType(_) => "DW_TAG_ptr_to_member_type",
            DIE::DwTagReferenceType(_) => "DW_TAG_reference_type",
            DIE::DwTagRestrictType(_) => "DW_TAG_restrict_type",
            DIE::DwTagRvalueReferenceType(_) => "DW_TAG_rvalue_reference_type",
            DIE::DwTagSetType(_) => "DW_TAG_set_type",
            DIE::DwTagSharedType(_) => "DW_TAG_shared_type",
            DIE::DwTagStringType(_) => "DW_TAG_string_type",
            DIE::DwTagStructureType(_) => "DW_TAG_structure_type",
            DIE::DwTagSubprogram(_) => "DW_TAG_subprogram",
            DIE::DwTagSubrangeType(_) => "DW_TAG_subrange_type",
            DIE::DwTagSubroutineType(_) => "DW_TAG_subroutine_type",
            DIE::DwTagTemplateAlias(_) => "DW_TAG_template_alias",
            DIE::DwTagTemplateTypeParameter(_) => "DW_TAG_template_type_parameter",
            DIE::DwTagTemplateValueParameter(_) => "DW_TAG_template_value_parameter",
            DIE::DwTagThrownType(_) => "DW_TAG_thrown_type",
            DIE::DwTagTryBlock(_) => "DW_TAG_try_block",
            DIE::DwTagTypedef(_) => "DW_TAG_typedef",
            DIE::DwTagTypeUnit(_) => "DW_TAG_type_unit",
            DIE::DwTagUnionType(_) => "DW_TAG_union_type",
            DIE::DwTagUnspecifiedParameters(_) => "DW_TAG_unspecified_parameters",
            DIE::DwTagUnspecifiedType(_) => "DW_TAG_unspecified_type",
            DIE::DwTagVariable(_) => "DW_TAG_variable",
            DIE::DwTagVariant(_) => "DW_TAG_variant",
            DIE::DwTagVariantPart(_) => "DW_TAG_variant_part",
            DIE::DwTagVolatileType(_) => "DW_TAG_volatile_type",
            DIE::DwTagWithStmt(_) => "DW_TAG_with_stmt",
        }
    }

    fn to_data_inner(self, context: &mut Context) -> Debug {
        match self {
            DIE::DwTagAccessDeclaration(info) => info.to_data(context),
            DIE::DwTagArrayType(info) => info.to_data(context),
            DIE::DwTagBaseType(info) => info.to_data(context),
            DIE::DwTagCatchBlock(info) => info.to_data(context),
            DIE::DwTagClassType(info) => info.to_data(context),
            DIE::DwTagCommonBlock(info) => info.to_data(context),
            DIE::DwTagCommonInclusion(info) => info.to_data(context),
            DIE::DwTagCondition(info) => info.to_data(context),
            DIE::DwTagConstType(info) => info.to_data(context),
            DIE::DwTagConstant(info) => info.to_data(context),
            DIE::DwTagCompileUnit(info) => info.to_data(context),
            DIE::DwTagDwarfProcedure(info) => info.to_data(context),
            DIE::DwTagEntryPoint(info) => info.to_data(context),
            DIE::DwTagEnumerationType(info) => info.to_data(context),
            DIE::DwTagEnumerator(info) => info.to_data(context),
            DIE::DwTagFileType(info) => info.to_data(context),
            DIE::DwTagFormalParameter(info) => info.to_data(context),
            DIE::DwTagFriend(info) => info.to_data(context),
            DIE::DwTagImportedDeclaration(info) => info.to_data(context),
            DIE::DwTagImportedModule(info) => info.to_data(context),
            DIE::DwTagImportedUnit(info) => info.to_data(context),
            DIE::DwTagInheritance(info) => info.to_data(context),
            DIE::DwTagInlinedSubroutine(info) => info.to_data(context),
            DIE::DwTagInterfaceType(info) => info.to_data(context),
            DIE::DwTagLabel(info) => info.to_data(context),
            DIE::DwTagLexicalBlock(info) => info.to_data(context),
            DIE::DwTagMember(info) => info.to_data(context),
            DIE::DwTagModule(info) => info.to_data(context),
            DIE::DwTagNamelist(info) => info.to_data(context),
            DIE::DwTagNamelistItem(info) => info.to_data(context),
            DIE::DwTagNamespace(info) => info.to_data(context),
            DIE::DwTagPackedType(info) => info.to_data(context),
            DIE::DwTagPartialUnit(info) => info.to_data(context),
            DIE::DwTagPointerType(info) => info.to_data(context),
            DIE::DwTagPtrToMemberType(info) => info.to_data(context),
            DIE::DwTagReferenceType(info) => info.to_data(context),
            DIE::DwTagRestrictType(info) => info.to_data(context),
            DIE::DwTagRvalueReferenceType(info) => info.to_data(context),
            DIE::DwTagSetType(info) => info.to_data(context),
            DIE::DwTagSharedType(info) => info.to_data(context),
            DIE::DwTagStringType(info) => info.to_data(context),
            DIE::DwTagStructureType(info) => info.to_data(context),
            DIE::DwTagSubprogram(info) => info.to_data(context),
            DIE::DwTagSubrangeType(info) => info.to_data(context),
            DIE::DwTagSubroutineType(info) => info.to_data(context),
            DIE::DwTagTemplateAlias(info) => info.to_data(context),
            DIE::DwTagTemplateTypeParameter(info) => info.to_data(context),
            DIE::DwTagTemplateValueParameter(info) => info.to_data(context),
            DIE::DwTagThrownType(info) => info.to_data(context),
            DIE::DwTagTryBlock(info) => info.to_data(context),
            DIE::DwTagTypeUnit(info) => info.to_data(context),
            DIE::DwTagTypedef(info) => info.to_data(context),
            DIE::DwTagUnionType(info) => info.to_data(context),
            DIE::DwTagUnspecifiedParameters(info) => info.to_data(context),
            DIE::DwTagUnspecifiedType(info) => info.to_data(context),
            DIE::DwTagVariable(info) => info.to_data(context),
            DIE::DwTagVariant(info) => info.to_data(context),
            DIE::DwTagVariantPart(info) => info.to_data(context),
            DIE::DwTagVolatileType(info) => info.to_data(context),
            DIE::DwTagWithStmt(info) => info.to_data(context),
        }
    }

    fn to_data(self, context: &mut Context) -> (Debug, &'static str, u8) {
        let tag_name = self.tag_name();
        let id = self.id();
        let debug = self.to_data_inner(context);
        (debug, tag_name, id)
    }
}

/// Structure representing a Dwarf debug information tree
pub struct DwarfDebug {
    pub info: DIE,
    pub childrens: Vec<DwarfDebug>,
}

impl DwarfDebug {
    fn to_data_inner(self, mut id: u8, context: &mut Context) -> (u8, Debug) {
        let (mut debug, tag_name, tag_id) = self.info.to_data(context);
        let has_childrens = self.childrens.is_empty();
        debug.abbrev = dubyte(id).add_comment("Abbreviation Code".to_string())
            + dubyte(tag_id).add_comment(tag_name.to_string())
            + if has_childrens {
                dubyte(0).add_comment("DW_CHILDREN_no".to_string())
            } else {
                dubyte(1).add_comment("DW_CHILDREN_yes".to_string())
            }
            + debug.abbrev;
        debug.info = dubyte(id).add_comment("Abbrev id".to_string()) + debug.info;
        id = id + 1;
        for child in self.childrens {
            let (new_id, new_debug) = child.to_data_inner(id, context);
            debug.concat(new_debug);
            id = new_id;
        }
        if has_childrens {
            debug.info += dbyte(0).add_comment("End of childrens".to_string());
        }
        (id, debug)
    }

    pub fn to_data(self, mut context: Context) -> super::DebugSegments {
        let (_, debug) = self.to_data_inner(0, &mut context);
        super::DebugSegments {
            debug_abbrev: debug.abbrev + dbyte(0).add_comment("EOM(3)".to_string()),
            debug_info: debug.info,
            debug_str: context.to_data(),
        }
    }
}
