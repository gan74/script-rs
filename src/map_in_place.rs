use std::boxed::Box;
use std::rc::Rc;
use std::vec::Vec;
use std::ops::*;
use std::ptr;

pub trait MapInPlace<T> {
    fn map_in_place<F: FnMut(T) -> T>(self, map: F) -> Self;
}

impl<T> MapInPlace<T> for Box<T> {
    fn map_in_place<F: FnMut(T) -> T>(self, mut map: F) -> Box<T> {
        unsafe  {
            let ptr = Box::into_raw(self);
            let content = ptr::read(ptr);
            let res = map(content);
            ptr::write(ptr, res);
            Box::from_raw(ptr)
        }
    }
}

impl<T: Clone> MapInPlace<T> for Rc<T> {
    fn map_in_place<F: FnMut(T) -> T>(mut self, mut map: F) -> Rc<T> {
        unsafe  {
            let ptr: *mut T = Rc::get_mut(&mut self).unwrap();
            let content = ptr::read(ptr);
            let res = map(content);
            ptr::write(ptr, res);
            Rc::from_raw(ptr)
        }
    }
}

impl<T> MapInPlace<T> for Vec<T> {
    fn map_in_place<F: FnMut(T) -> T>(mut self, mut map: F) -> Vec<T> {
        unsafe  {
            let ptr = self.as_mut_ptr();
            for i in 0..self.len() {
                let elem = ptr.offset(i as isize);
                let content = ptr::read(elem);
                let res = map(content);
                ptr::write(elem, res);
            }
            self
        }
    }
}



/*pub trait TryMapBoxInPlace<T> {
    fn try_map_in_place<Y, F: FnMut(T) -> Y>(self, map: F) -> Box<Y>;
}

impl<T> TryMapBoxInPlace<T> for Box<T> {
    fn try_map_in_place<Y, F: FnMut(T) -> Y>(self, mut map: F) -> Box<Y> {
        Box::new(map(*self))
    }
}

pub trait TryMapRcInPlace<T> {
    fn try_map_in_place<Y, F: FnMut(T) -> Y>(self, map: F) -> Rc<Y>;
}

impl<T: Clone> TryMapRcInPlace<T> for Rc<T> {
    fn try_map_in_place<Y, F: FnMut(T) -> Y>(mut self, mut map: F) -> Rc<Y> {
        Rc::make_mut(&mut self);
        Rc::new(map(Rc::try_unwrap(self).ok().unwrap()))
    }
}


pub trait TryMapVecInPlace<T> {
    fn try_map_in_place<Y, F: FnMut(T) -> Y>(self, map: F) -> Vec<Y>;
}

impl<T> TryMapVecInPlace<T> for Vec<T> {
    fn try_map_in_place<Y, F: FnMut(T) -> Y>(self, map: F) -> Vec<Y> {
        self.into_iter().map(map).collect()
    }
}*/
