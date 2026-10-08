//! Constants of the page runs of PolyASM heaps.

/// PolyASM requests of at least this many bytes start on a
/// [`PAGE_RUN_ALIGN`] boundary. A runtime maps immutable file pages over such
/// buffers (a read of a stored object lands as a file mapping, and a copy of
/// one gets re-backed by it), and a file page lands only on a guest page at
/// the same offset, so a run that starts on a page boundary takes every
/// whole page of a file it holds from byte zero.
pub(crate) const PAGE_RUN: usize = 1 << 20;

/// The boundary a page run starts on: the widest host page a PolyASM lane
/// runs on, so the run starts on a page boundary of every host.
pub(crate) const PAGE_RUN_ALIGN: usize = 1 << 16;
