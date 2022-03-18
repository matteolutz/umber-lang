use crate::compiler::Compiler;

#[test]
pub fn compiler_register_distr() {
    let mut c = Compiler::new();

    let i1 = c.res_reg();
    println!("Index is: {}, regs: {:#08b}", i1, c.gen_regs());

    let i2 = c.res_reg();
    println!("Index is: {}, regs: {:#08b}", i2, c.gen_regs());

    let i3 = c.res_reg();
    println!("Index is: {}, regs: {:#08b}", i3, c.gen_regs());

    c.free_reg(0);
    println!("Regs: {:#08b}", c.gen_regs());

    assert_eq!(1, 2);
}