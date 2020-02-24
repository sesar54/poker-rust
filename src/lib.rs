//! A library for poker games
//!
//!
#![feature(proc_macro_hygiene)]
#![feature(drain_filter)]
#![warn(unused_import_braces, unused_qualifications, trivial_casts)]
#![warn(trivial_numeric_casts, private_in_public, variant_size_differences)]
#![warn(stable_features, unreachable_pub, non_shorthand_field_patterns)]
//#![warn(unused_attributes, unused_imports, unused_mut)] // missing_docs
#![warn(renamed_and_removed_lints, stable_features, unused_allocation)]
#![warn(unused_comparisons, bare_trait_objects, unused_must_use, const_err)]
#![allow(unused_imports)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate num_traits;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
extern crate seq_macro;
extern crate variant_count;
#[macro_use]
extern crate mimpl;
extern crate adjacent_pair_iterator;
#[macro_use]
extern crate arrayref;
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
