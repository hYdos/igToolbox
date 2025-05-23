use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_fs::Endian;
use crate::core::ig_objects::igAny;
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use log::{error, warn};
use std::any::TypeId;
use std::io::{Cursor, Read, Write};
use std::sync::{Arc, RwLock};
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;

pub struct igPlaceholderMetafield(pub u32, pub Arc<str>);

impl igMetaField for igPlaceholderMetafield {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Vec<u8>>()
    }

    fn value_from_igz(
        &self,
        handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        ctx: &IgzLoaderContext,
        _registry: &igMetafieldRegistry
    ) -> Option<igAny> {
        warn!("{} has no implementation. Using igPlaceholderMetafield. Harass hydos to implement this or make a PR!", self.1);
        let mut fake_buffer = Vec::with_capacity(self.platform_size(ctx.platform.clone()) as usize);
        handle.read_exact(&mut fake_buffer).unwrap();
        Some(Arc::new(RwLock::new(fake_buffer)))
    }

    fn value_into_igz(
        &self,
        handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        ctx: &mut IgzSaverContext,
    ) -> Result<(), IgzSaverError> {
        let fake_buffer = Vec::with_capacity(self.platform_size(ctx.platform.clone()) as usize);
        handle
            .write(fake_buffer.as_slice())
            .map_err(|_e| IgzSaverError::Unknown)
            .map(|_t| {})
    }

    fn value_from_igx(
        &self,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        _ctx: &mut IgxLoaderContext,
    ) -> Option<igAny> {
        todo!()
    }

    fn value_into_igx(
        &self,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        _ctx: &mut IgxSaverContext,
    ) -> Result<(), IgxSaverError> {
        error!(
            "Using igPlaceholderMetafield for saving is not supported. Implement the metafield!"
        );
        panic!("Alchemy Error! Check the logs.")
    }

    fn value_from_igb(
        &self,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        _ctx: &mut IgbLoaderContext,
    ) -> Option<igAny> {
        error!("Using igPlaceholderMetafield not supported. Implement the metafield!");
        panic!("Alchemy Error! Check the logs.")
    }

    fn value_into_igb(
        &self,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: &Endian,
        _ctx: &mut IgbSaverContext,
    ) -> Result<(), IgbSaverError> {
        error!(
            "Using igPlaceholderMetafield for saving is not supported. Implement the metafield!"
        );
        panic!("Alchemy Error! Check the logs.")
    }

    fn platform_size(&self, _platform: IG_CORE_PLATFORM) -> u32 {
        self.0
    }

    fn platform_alignment(&self, _platform: IG_CORE_PLATFORM) -> u32 {
        self.0
    }
}
