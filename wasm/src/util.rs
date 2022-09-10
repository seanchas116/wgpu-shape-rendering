pub fn cast_to_u8_slice<T>(xs: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            xs.as_ptr() as *const u8,
            xs.len() * std::mem::size_of::<T>(),
        )
    }
}
