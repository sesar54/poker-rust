//! A library for poker games

#![feature(drain_filter,exclusive_range_pattern)]
#![feature(let_chains)]

#![warn(unused_import_braces, unused_qualifications, trivial_casts)]
#![warn(trivial_numeric_casts, private_in_public, variant_size_differences)]
#![warn(stable_features, unreachable_pub, non_shorthand_field_patterns)]
#![warn(unused_attributes, unused_imports, unused_mut)] // missing_docs
#![warn(renamed_and_removed_lints, stable_features, unused_allocation)]
#![warn(unused_comparisons, bare_trait_objects, unused_must_use, const_err)]

//#![forbid(unsafe_code)]

// Help Modules
#[macro_use]
pub mod macros;

// Basic Modules
pub mod card;
pub mod deck;

// Game Modules
pub mod holdem;

// Prelude
pub use card::{ Suit::*, Value::*, * };
pub use deck::*;
pub use macros::*;
