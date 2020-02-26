//! A library for poker games
//!
//!

#![feature(box_syntax)]
#![feature(proc_macro_hygiene)]
#![feature(half_open_range_patterns)]
#![feature(drain_filter)]
#![warn(unused_import_braces, unused_qualifications, trivial_casts)]
#![warn(trivial_numeric_casts, private_in_public, variant_size_differences)]
#![warn(stable_features, unreachable_pub, non_shorthand_field_patterns)]
//#![warn(unused_attributes, unused_imports, unused_mut)] // missing_docs
#![warn(renamed_and_removed_lints, stable_features, unused_allocation)]
#![warn(unused_comparisons, bare_trait_objects, unused_must_use, const_err)]
#![allow(unused_imports, dead_code)]
//#![forbid(unsafe_code)]

extern crate custom_derive;
extern crate enum_derive;
extern crate log;
extern crate mimpl;
extern crate num_derive;
extern crate num_traits;
extern crate seq_macro;
extern crate strum;
extern crate strum_macros;
extern crate variant_count;

//pub mod bot; // Included bots
//pub mod betting // Chance and gameplay statistics
//pub mod score // Player statistics
#[macro_use]
pub mod card; // Structure
pub mod deck; // Structure
#[macro_use]
pub mod hand; // Poker logic
pub mod player; // Player interface
pub mod prelude;
pub mod table; // Gameplay logic
