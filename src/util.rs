

pub fn ref_to_slice<T>(p: &T) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            (p as *const T) as *const u8,
            std::mem::size_of::<T>()
        )
    }
}

pub fn slice_to_slice<T>(p: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            p.as_ptr() as *const u8,
            p.len() * std::mem::size_of::<T>()
        )
    }
}
