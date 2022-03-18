use crate::compiler::Compiler;

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