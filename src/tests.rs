use crate::compiler::Compiler;
use crate::semantics::Validator;
use crate::symbol_table::Symbol;
use crate::values::value_type::ValueTypes;
use crate::values::value_type::void_type::VoidType;
use crate::values::vtype::ValueTypes;

#[test]
pub fn compiler_register_distribution() {
    let mut c = Compiler::new();

    c.res_scratch();
    assert_eq!(*c.scratch_regs(), 0b0000001);

    c.res_scratch();
    assert_eq!(*c.scratch_regs(), 0b0000011);

    c.res_scratch();
    assert_eq!(*c.scratch_regs(), 0b0000111);

    c.free_scratch(1);
    assert_eq!(*c.scratch_regs(), 0b0000101);

    c.free_scratch(2);
    assert_eq!(*c.scratch_regs(), 0b0000001);

    c.free_scratch(0);
    assert_eq!(*c.scratch_regs(), 0b0000000);

    c.free_scratch(5);
    assert_eq!(*c.scratch_regs(), 0b0000000);

}

#[test]
pub fn semantics_symbol_stack() {
    let mut v = Validator::new();

    v.declare_symbol("a", Symbol::new(Box::new(VoidType::new()), false));

    assert_eq!(v.has_symbol("a"), true);
    assert_eq!(v.get_symbol("a").unwrap().value_type().value_type(), ValueTypes::Void);
    assert_eq!(v.is_symbol_mut("a"), false);

    v.push_child_table();

    assert_eq!(v.has_symbol("a"), true);
    assert_eq!(v.has_symbol("b"), false);
    v.declare_symbol("b", Symbol::new(Box::new(VoidType::new()), false));
    assert_eq!(v.has_symbol("b"), true);

    v.pop_child_table();

    assert_eq!(v.has_symbol("a"), true);
    assert_eq!(v.has_symbol("b"), false);
}