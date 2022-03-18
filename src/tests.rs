use crate::compiler::Compiler;
use crate::semantics::Validator;
use crate::symboltable::Symbol;
use crate::values::types::void::VoidType;
use crate::values::vtype::ValueTypes;

#[test]
pub fn compiler_register_distr() {
    let mut c = Compiler::new();

    c.res_reg();
    assert_eq!(*c.gen_regs(), 0b0000001);

    c.res_reg();
    assert_eq!(*c.gen_regs(), 0b0000011);

    c.res_reg();
    assert_eq!(*c.gen_regs(), 0b0000111);

    c.free_reg(1);
    assert_eq!(*c.gen_regs(), 0b0000101);

    c.free_reg(2);
    assert_eq!(*c.gen_regs(), 0b0000001);

    c.free_reg(0);
    assert_eq!(*c.gen_regs(), 0b0000000);

    c.free_reg(5);
    assert_eq!(*c.gen_regs(), 0b0000000);

}

#[test]
pub fn semantics_symbol_stack() {
    let mut v = Validator::new();

    v.declare("a", Symbol::new(Box::new(VoidType::new()), false));

    assert_eq!(v.has_symbol("a"), true);
    assert_eq!(v.get_symbol("a").unwrap().value_type().value_type(), ValueTypes::Void);
    assert_eq!(v.is_symbol_mut("a"), false);
}