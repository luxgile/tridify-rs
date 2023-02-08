use std::ops::DerefMut;
use std::{
    cell::RefCell,
    marker::Unsize,
    ops::{CoerceUnsized, Deref},
    rc::Rc,
};

pub struct AssetRef<T: ?Sized> {
    pub asset: Rc<RefCell<T>>,
}
impl<T: 'static> AssetRef<T> {
    pub fn new(asset: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(asset))
        // Self {
        //     asset: Rc::new(RefCell::new(asset)),
        // }
    }
}
impl<T> Clone for AssetRef<T> {
    fn clone(&self) -> Self {
        Self {
            asset: Rc::clone(&self.asset),
        }
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<AssetRef<U>> for AssetRef<T> {
}
