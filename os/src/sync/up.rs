//! Uniprocessor interior mutability primitives
use core::cell::{RefCell, RefMut};

/// Wrap a static data structure inside it so that we are
/// able to access it without any `unsafe`.
///
/// We should only use it in uniprocessor.
///
/// In order to get mutable reference of inner data, call
/// `exclusive_access`.
///定义了这个结构体，单处理器内部内部的XX
pub struct UPSafeCell<T> {
    /// inner data
    ///提供内部可变性，允许不用unsafe
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}
///这个可以在多线程中使用
impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in
    /// uniprocessor.
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    /// Panic if the data has been borrowed.
    ///提供必要的内存空间
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
