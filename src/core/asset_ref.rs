
use std::{cell::RefCell, rc::Rc, ops::{Deref, CoerceUnsized}, marker::Unsize};
use std::ops::DerefMut;


pub struct AssetRef<T: ?Sized> {
    asset: Rc<RefCell<T>>,
}
impl<T> AssetRef<T> {
    pub fn new(asset: T) -> Self {
        Self {
            asset: Rc::new(RefCell::new(asset))
        }
    }
}
impl<T> Clone for AssetRef<T> {
    fn clone(&self) -> Self {
        Self {
            asset: Rc::clone(&self.asset)
        }
    }
}
impl<T> Deref for AssetRef<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        self.asset.as_ref()
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<AssetRef<U>> for AssetRef<T> {}
