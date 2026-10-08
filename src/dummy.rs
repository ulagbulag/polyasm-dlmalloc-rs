use crate::Allocator;
use core::ptr;

pub struct System {
    _priv: (),
}

impl System {
    pub const fn new() -> System {
        System { _priv: () }
    }
}

unsafe impl Allocator for System {
    fn alloc(&self, _size: usize) -> Option<(ptr::NonNull<u8>, usize, u32)> {
        None
    }

    fn remap(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8 {
        ptr::null_mut()
    }

    fn free_part(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool {
        false
    }

    fn free(&self, _ptr: *mut u8, _size: usize) -> bool {
        false
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        false
    }

    fn page_size(&self) -> usize {
        1
    }
}

#[cfg(feature = "global")]
pub fn acquire_global_lock() {
    // A target without a platform has no second thread to lock against.
}

#[cfg(feature = "global")]
pub fn release_global_lock() {
    // As in `acquire_global_lock`.
}

#[cfg(feature = "global")]
pub unsafe fn enable_alloc_after_fork() {
    // A target without a platform runs as one process.
}
