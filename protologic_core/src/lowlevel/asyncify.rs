use std::alloc::Layout;

#[no_mangle]
extern fn asyncify_malloc_buffer(size: i32) -> i32
{
    if size < 0 {
        return -1;
    }

    if let Ok(layout) = Layout::from_size_align(size as usize, 8)
    {
        unsafe
        {
            return std::alloc::alloc(layout) as i32;
        }
    }
        
    return -1;
}

#[no_mangle]
extern fn asyncify_free_buffer(ptr: i32, size: i32)
{
    // Using "unwrap" here because the layout was successfully created with these
    // parameters when it was first allocated, so therefore it's ok now!
    let layout = Layout::from_size_align(size as usize, 8).unwrap();
    
    unsafe
    {
        std::alloc::dealloc(ptr as *mut u8, layout);
    }
}