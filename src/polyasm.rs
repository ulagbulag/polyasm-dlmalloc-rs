use crate::Allocator;
use core::cell::Cell;
use core::ptr;

::polyasm_format::heap_boundaries!(HEAP_BASE, HEAP_END);

#[doc = "The system allocator PolyASM donates its linked heap through."]
pub struct System {
    donated: Cell<bool>,
}

unsafe impl Sync for System {}

impl System {
    #[doc = "Answers a system allocator that holds its heap for donation."]
    pub const fn new() -> System {
        System {
            donated: Cell::new(false),
        }
    }
}

/// Answers the one heap the linker fixed into this image.
///
/// PolyASM carries WebAssembly's family with a heap fixed at link time: the
/// linker decides the whole heap and names it by the two PolyASM boundary
/// symbols. The allocator takes it once and answers `None` after, since a
/// second donation would hand out the same bytes twice.
fn linked_heap(size: usize) -> Option<(usize, usize)> {
    let base = (&raw const HEAP_BASE) as usize;
    let end = (&raw const HEAP_END) as usize;
    if base == 0 || end <= base {
        return None;
    }
    let len = end - base;
    if len < size {
        None
    } else {
        Some((base, len))
    }
}

unsafe impl Allocator for System {
    fn alloc(&self, size: usize) -> Option<(ptr::NonNull<u8>, usize, u32)> {
        if size == 0 {
            return None;
        }
        if self.donated.get() {
            return None;
        }
        let (base, len) = linked_heap(size)?;
        let base = ptr::NonNull::new(base as *mut u8)?;
        self.donated.set(true);
        Some((base, len, 0))
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

    /// Answers the page the linked heap is measured in, 64 KiB.
    ///
    /// dlmalloc reads this on the paths that size a segment, so it answers a
    /// page; the 16-byte malloc alignment serves a different purpose. The
    /// heap boundaries above come from `polyasm-format`, the authority on the
    /// layout of a PolyASM image.
    fn page_size(&self) -> usize {
        64 * 1024
    }
}

#[cfg(feature = "global")]
#[doc = "Takes the global allocator lock, an empty call on this target."]
pub fn acquire_global_lock() {}

#[cfg(feature = "global")]
#[doc = "Releases the global allocator lock, an empty call on this target."]
pub fn release_global_lock() {}

#[cfg(feature = "global")]
#[doc = "Reopens allocation after a fork, an empty call on this target."]
#[doc = ""]
#[doc = "# Safety"]
#[doc = ""]
#[doc = "The caller pledges the process is single-threaded, as after a fork."]
pub unsafe fn enable_alloc_after_fork() {}
