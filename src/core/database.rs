use std::{any::Any, cell::RefCell, collections::HashMap, marker::PhantomData};
use uuid::Uuid;

use crate::Texture;

pub struct Database<T: Asset> {
    table: HashMap<Uuid, T>,
}
impl<T: Asset> Database<T> {
    pub fn new() -> Self {
        Database {
            table: HashMap::new(),
        }
    }
    pub fn add_asset(&mut self, asset: T) -> AssetRef<T> {
        let id = Uuid::new_v4();
        let asset_ref = AssetRef::new(id);
        self.table.insert(id, asset);
        asset_ref
    }

    pub fn get(&self, id: &AssetRef<T>) -> Option<&T> {
        self.table.get(&id.id)
    }
    pub fn get_mut(&mut self, id: &AssetRef<T>) -> Option<&mut T> {
        self.table.get_mut(&id.id)
    }
}

pub trait Asset {
    fn as_any(&self) -> &dyn Any;
}

pub struct AssetRef<T: Asset> {
    id: Uuid,
    marker: PhantomData<fn() -> T>,
}
impl<T: Asset> AssetRef<T> {
    pub fn new(id: Uuid) -> Self {
        AssetRef {
            id,
            marker: PhantomData,
        }
    }
}
impl<T: Asset> Clone for AssetRef<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            marker: PhantomData,
        }
    }
}
