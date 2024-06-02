#![warn(missing_docs)]

//! Quick and dirty "named asset" system

use std::sync::Mutex;
use bevy::{prelude::*, utils::HashMap};
use once_cell::sync::Lazy;

/// A hashmap which stores strings pointing to asset handles
pub static ASSET_MAP: Lazy<Mutex<HashMap<String, UntypedHandle>>> = Lazy::new(|| Mutex::new(HashMap::new()));
