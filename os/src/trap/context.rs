use riscv::register::sstatus::{self, Sstatus, SPP};
///中断和异常的控制
#[repr(C)]
#[derive(Debug)]
/// trap context structure containing sstatus, sepc and registers
pub struct TrapContext {
    /// General-Purpose Register x0-31
    ///表示32个寄存器
    pub x: [usize; 32],
    /// Supervisor Status Register
    ///表示监管者寄存器
    pub sstatus: Sstatus,
    /// Supervisor Exception Program Counter
    ///表示监管者程序寄存器
    pub sepc: usize,
}

impl TrapContext {
    /// put the sp(stack pointer) into x\[2\] field of TrapContext
    ///把栈存储到x寄存器中，存到x2字段内
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// init the trap context of an application
    ///初始化应用程序的异常上下文
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read(); // CSR sstatus
        sstatus.set_spp(SPP::User); //previous privilege mode: user mode
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}
