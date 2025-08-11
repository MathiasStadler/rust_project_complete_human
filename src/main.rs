// In your main.rs

//#[cfg(feature = "dhat-heap")]
//#[global_allocator]
//static ALLOC: dhat::Alloc = dhat::Alloc;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    println!("Hello, world!");
}

/*
// In your main.rs
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let _profiler = dhat::Profiler::new_heap();
    
    // Your program code
}
*/