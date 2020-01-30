//! A library for poker games
//!
//!

#![feature(proc_macro_hygiene)]
#![feature(drain_filter)]
#![warn(unused_import_braces, unused_qualifications, trivial_casts)]
#![warn(trivial_numeric_casts, private_in_public, variant_size_differences)]
#![warn(stable_features, unreachable_pub, non_shorthand_field_patterns)]
#![warn(unused_attributes, unused_imports, unused_mut)] // missing_docs
#![warn(renamed_and_removed_lints, stable_features, unused_allocation)]
#![warn(unused_comparisons, bare_trait_objects, unused_must_use, const_err)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate num_derive;
extern crate num_traits;
extern crate seq_macro;
extern crate variant_count;
#[macro_use]
extern crate mimpl;

// Help Modules
#[macro_use]
pub mod macros;

// Basic Modules
//pub mod bot;
pub mod card;
pub mod deck;
pub mod hand;

pub mod player;
pub mod table;

pub mod prelude;
