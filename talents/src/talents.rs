/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

extern crate alloc;

use crate::behavior::*;
use crate::capability::*;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use core::any::*;
use ruspiro_core::{error::*, lock::*, singleton::Singleton};

/// Managing the Talents of the RusPiRo
#[derive(Debug)]
pub struct Talents {
    inner: _Talents,
}

impl Talents {
    pub fn default() -> Self {
        Self {
            inner: _Talents {
                capabilities: BTreeMap::new(),
                behaviors: BTreeMap::new(),
            },
        }
    }

    pub fn add_capability<T>(&mut self, capability: T)
    where
        T: 'static + Capability + Any,
    {
        self.inner
            .capabilities
            .insert(TypeId::of::<T>(), DataLock::new(Box::from(capability)));
    }

    pub fn add_behavior<T>(&mut self, behavior: T)
    where
        T: 'static + Behavior + Any,
    {
        self.inner
            .behaviors
            .insert(TypeId::of::<T>(), DataLock::new(Box::from(behavior)));
    }

    pub fn use_capability<T, F, R>(&self, f: F) -> Result<R, BoxError>
    where
        T: 'static + Capability + Any,
        F: Fn(&T) -> R,
    {
        let capability = self.inner.get_capability::<T>()?.read();

        Ok(f(capability.downcast_ref::<T>().unwrap()))
    }

    pub fn use_capability_mut<T, F, R>(&self, f: F) -> Result<R, BoxError>
    where
        T: 'static + Capability + Any,
        F: FnOnce(&mut T) -> R,
    {
        let capability = self.inner.get_capability::<T>()?;

        if let Some(ref mut locked_capability) = capability.try_lock() {
            Ok(f(locked_capability.downcast_mut::<T>().unwrap()))
        } else {
            Err(GenericError::with_message("unable to lock the capability for usage").into())
        }
    }
}

#[derive(Debug)]
struct _Talents {
    capabilities: BTreeMap<TypeId, DataLock<Box<dyn Any>>>,
    behaviors: BTreeMap<TypeId, DataLock<Box<dyn Any>>>,
}

impl _Talents {
    fn get_capability<T>(&self) -> Result<&DataLock<Box<dyn Any>>, BoxError>
    where
        T: Sized + 'static + Capability + Any,
    {
        match self.capabilities.get(&TypeId::of::<T>()) {
            Some(capability) => Ok(capability),
            _ => Err(GenericError::with_message(
                format!("capability {:?} unknown", TypeId::of::<T>()).as_str(),
            )
            .into()),
        }
    }
}

/*
impl<T> Capability for &Singleton<T>
    where T: Capability {

}
*/
