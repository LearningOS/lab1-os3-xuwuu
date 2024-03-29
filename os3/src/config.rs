pub const USER_STACK_SIZE: usize = 8192;
pub const KERNEL_STACK_SIZE: usize = 8192 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x20000;
pub const MAX_APP_NUM: usize = 16;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;
pub const CLOCK_FREQ: usize = 12500000;
pub const MAX_SYSCALL_NUM: usize = 500;