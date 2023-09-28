//! MSYX Server
//! Music Stream YX Server
//!
//!

/// This module handles the database and data structures
pub mod data;
/// This module contains the data models
pub mod model;
/// This module provides tools for scanning Audio files
pub mod scanner;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::{model::*, scanner::*};
    pub use anyhow::{Error, Ok, Result};
    pub use std::{
        cell::RefCell,
        collections::HashMap,
        env,
        fs::{self},
        path::{self, Path, PathBuf},
        process::exit,
        thread,
    };
    pub use symphonia::core::{
        codecs::{DecoderOptions, CODEC_TYPE_NULL},
        formats::FormatOptions,
        io::MediaSourceStream,
        meta::{self, MetadataOptions, StandardTagKey, Tag},
        probe::{Hint, ProbeResult},
    };
}
