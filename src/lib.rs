
pub struct PackedBitStack {
    vec: Vec<u8>
}

use std::mem::size_of;

macro_rules! impl_ps {
    ($push_name: ident, $pop_name: ident, $type: ty) => {
        pub fn $push_name(&mut self, value: $type) {
            for &bits in &value.to_be_bytes() {
                self.vec.push(bits);
            }
        }
    
        pub fn $pop_name(&mut self) -> $type {
            const Z: usize = size_of::<$type>();
            let mut array = [0; Z];
            for i in 0..Z {
                array[Z - i - 1] = self.vec.pop().unwrap();
            }
            <$type>::from_be_bytes(array)
        }
    };
}

impl PackedBitStack {
    pub fn new() -> Self {
        PackedBitStack {
            vec: vec![]
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
    
    impl_ps!(push_u8,       pop_u8,     u8);
    impl_ps!(push_u16,      pop_u16,    u16);
    impl_ps!(push_u32,      pop_u32,    u32);
    impl_ps!(push_u64,      pop_u64,    u64);
    impl_ps!(push_u128,     pop_u128,   u128);
    impl_ps!(push_usize,    pop_usize,  usize);

    impl_ps!(push_i8,       pop_i8,     i8);
    impl_ps!(push_i16,      pop_i16,    i16);
    impl_ps!(push_i32,      pop_i32,    i32);
    impl_ps!(push_i64,      pop_i64,    i64);
    impl_ps!(push_i128,     pop_i128,   i128);
    impl_ps!(push_isize,    pop_isize,  isize);

    pub unsafe fn push_heap<T>(&mut self, value: T) {
        self.push_usize(Box::into_raw(Box::new(value)) as usize)
    }

    pub unsafe fn pop_heap<T>(&mut self) -> T {
        *Box::from_raw(self.pop_usize() as *mut T)
    }

    pub unsafe fn push_bits<T>(&mut self, value: T) {
        let bitss = std::slice::from_raw_parts(
            (&value as *const T) as *const u8,
            size_of::<T>(),
        );
        for &bits in bitss {
            self.vec.push(bits);
        }
    }

    pub unsafe fn pop_bits<T>(&mut self) -> T {
        let z: usize = size_of::<T>();

        let mut array = vec![0; z];
        for i in 0..z {
            array[z - i - 1] = self.vec.pop().unwrap();
        }

        std::ptr::read(array.as_ptr() as *const T)
    }
}