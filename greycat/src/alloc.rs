// use std::alloc::{GlobalAlloc, Layout};

// /// GreyCat's allocator that implements
// /// `GlobalAlloc` using `gc_malloc_tls` and `gc_free_tls`
// ///
// /// ## Usage
// /// Add this lines to your `lib.rs` or `main.rs` to override the global allocator:
// /// ```rust
// /// use greycat::alloc::GreyCatAlloc;
// /// 
// /// #[global_allocator]
// /// static GLOBAL: GreyCatAlloc = GreyCatAlloc;
// /// ```
// pub struct GreyCatAlloc;

// unsafe impl GlobalAlloc for GreyCatAlloc {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         greycat_sys::gc_malloc_tls(layout.size())
//     }

//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         greycat_sys::gc_free_tls(ptr, layout.size())
//     }
// }
