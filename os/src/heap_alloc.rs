//! The global allocator
///堆内存分配器（简易）
use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;

#[global_allocator]
/// heap allocator instance
///实现了一个自旋锁，可以直接使用rust语言里有关的特性去管理堆内存
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

/// heap space ([u8; KERNEL_HEAP_SIZE])
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

/// initiate heap allocator
///初始化
pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[alloc_error_handler]
/// panic when heap allocation error occurs
///错误处理函数
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
