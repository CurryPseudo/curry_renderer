use crate::*;

pub trait AsArray<T> {
    fn as_array(self) -> T;
}

impl AsArray<[usize; 2]> for UVec2 {
    fn as_array(self) -> [usize; 2] {
        [self.x as usize, self.y as usize]
    }
}
