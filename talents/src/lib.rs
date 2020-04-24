/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-talents/0.1.0")]
#![cfg_attr(not(any(test, doctest)), no_std)]

//! Talents of the RusPiRo kernel to be available during the runtime of it
//! The Talents of the RusPiRo are the sum of all it's capabilities and behaviors
//! The [Capability] is here intended to provide a specific functionionality like accessing peripherals
//! attached. Examples could be GPIO, I²C, any kind of sensors.
//! The [Behavior] typically makes use of the provided [Capability] to provide specific actions, movements or results
//! from computations based on them.

/// Behavior trait
pub mod behavior;

/// Capability trait
pub mod capability;

/// The talents managing structure
mod talents;
pub use talents::*;
