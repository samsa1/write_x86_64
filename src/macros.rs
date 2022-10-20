
macro_rules! def_regq {
    ($name1:ident, $name2:ident) => {
        /// 64-bits registers (8 bytes)
        pub const $name1 : reg::RegQ = reg::RegQ::$name2;
    };
}

macro_rules! def_regl {
    ($name1:ident, $name2:ident) => {
        /// 32-bits registers (4 bytes)
        pub const $name1 : reg::RegL = reg::RegL::$name2;
    };
}

macro_rules! def_regw {
    ($name1:ident, $name2:ident) => {
        /// 16-bits registers (2 bytes)
        pub const $name1 : reg::RegW = reg::RegW::$name2;
    };
}

macro_rules! def_regb {
    ($name1:ident, $name2:ident) => {
        /// 8-bits registers (1 byte)
        pub const $name1 : reg::RegB = reg::RegB::$name2;
    };
}

macro_rules! build_instr_op_op {
    ($op:ident, $nameb:ident, $namew:ident, $namel:ident, $nameq:ident) => {
        /// Instructions between 1-bytes operands
        pub fn $nameb(reg1: reg::Operand<reg::RegB>, reg2: reg::Operand<reg::RegB>) -> Asm {
            Asm::Instr(Box::new(instr::InstrOpOp::new(
                instr::OpOpInstrName::$op,
                reg1,
                reg2,
            )))
        }

        build_instr_op_op!($op, $namew, $namel, $nameq);
    };

    ($op:ident, $namew:ident, $namel:ident, $nameq:ident) => {
        /// Instructions between 2-bytes operands
        pub fn $namew(reg1: reg::Operand<reg::RegW>, reg2: reg::Operand<reg::RegW>) -> Asm {
            Asm::Instr(Box::new(instr::InstrOpOp::new(
                instr::OpOpInstrName::$op,
                reg1,
                reg2,
            )))
        }

        build_instr_op_op!($op, $namel, $nameq);
    };

    ($op:ident, $namel:ident, $nameq:ident) => {
        /// Instructions between 4-bytes operands
        pub fn $namel(reg1: reg::Operand<reg::RegL>, reg2: reg::Operand<reg::RegL>) -> Asm {
            Asm::Instr(Box::new(instr::InstrOpOp::new(
                instr::OpOpInstrName::$op,
                reg1,
                reg2,
            )))
        }

        build_instr_op_op!($op, $nameq);
    };

    ($op:ident, $nameq:ident) => {
        /// Instructions between 8-bytes operands
        pub fn $nameq(reg1: reg::Operand<reg::RegQ>, reg2: reg::Operand<reg::RegQ>) -> Asm {
            Asm::Instr(Box::new(instr::InstrOpOp::new(
                instr::OpOpInstrName::$op,
                reg1,
                reg2,
            )))
        }
    };
}

macro_rules! build_instr_op_reg {
    ($op:ident, $nameb:ident, $namew:ident, $namel:ident, $nameq:ident) => {
        /// Instructions between 1-bytes operands
        pub fn $nameb(reg1: reg::Operand<reg::RegB>, reg2: reg::RegB) -> Asm {
            Asm::Instr(Box::new(instr::InstrOpOp::new(
                instr::OpOpInstrName::$op,
                reg1,
                reg::Operand::Reg(reg2),
            )))
        }

        build_instr_op_reg!($op, $namew, $namel, $nameq);
    };

    ($op:ident, $namew:ident, $namel:ident, $nameq:ident) => {
        /// Instructions between 2-bytes operands
        pub fn $namew(reg1: reg::Operand<reg::RegW>, reg2: reg::RegW) -> Asm {
            Asm::Instr(Box::new(instr::InstrOpOp::new(
                instr::OpOpInstrName::$op,
                reg1,
                reg::Operand::Reg(reg2),
            )))
        }

        build_instr_op_reg!($op, $namel, $nameq);
    };

    ($op:ident, $namel:ident, $nameq:ident) => {
        /// Instructions between 4-bytes operands
        pub fn $namel(reg1: reg::Operand<reg::RegL>, reg2: reg::RegL) -> Asm {
            Asm::Instr(Box::new(instr::InstrOpOp::new(
                instr::OpOpInstrName::$op,
                reg1,
                reg::Operand::Reg(reg2),
            )))
        }

        build_instr_op_reg!($op, $nameq);
    };

    ($op:ident, $nameq:ident) => {
        /// Instructions between 8-bytes operands
        pub fn $nameq(reg1: reg::Operand<reg::RegQ>, reg2 : reg::RegQ) -> Asm {
            Asm::Instr(Box::new(instr::InstrOpOp::new(
                instr::OpOpInstrName::$op,
                reg1,
                reg::Operand::Reg(reg2),
            )))
        }
    };
}


macro_rules! build_instr_op {
    ($op:ident, $nameb:ident, $namew:ident, $namel:ident, $nameq:ident) => {
        /// Instructions on 1-bytes operands
        pub fn $nameb(reg: reg::Operand<reg::RegB>) -> Asm {
            Asm::Instr(Box::new(instr::InstrOp::new(instr::OpInstrName::$op, reg)))
        }

        build_instr_op!($op, $namew, $namel, $nameq);
    };

    ($op:ident, $namew:ident, $namel:ident, $nameq:ident) => {
        /// Instructions on 2-bytes operands
        pub fn $namew(reg: reg::Operand<reg::RegW>) -> Asm {
            Asm::Instr(Box::new(instr::InstrOp::new(instr::OpInstrName::$op, reg)))
        }
        build_instr_op!($op, $namel, $nameq);
    };    

    ($op:ident, $namel:ident, $nameq:ident) => {
        /// Instructions on 4-bytes operands
        pub fn $namel(reg: reg::Operand<reg::RegL>) -> Asm {
            Asm::Instr(Box::new(instr::InstrOp::new(instr::OpInstrName::$op, reg)))
        }

        build_instr_op!($op, $nameq);
    };

    ($op:ident, $nameq:ident) => {

        /// Instructions on 8-bytes operands
        pub fn $nameq(reg: reg::Operand<reg::RegQ>) -> Asm {
            Asm::Instr(Box::new(instr::InstrOp::new(instr::OpInstrName::$op, reg)))
        }
    };

}


