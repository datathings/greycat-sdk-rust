// Use GreyCat allocator instead of default
#[global_allocator]
static GLOBAL: greycat::GreyCatAlloc = greycat::GreyCatAlloc;

mod bindings;
mod http2;