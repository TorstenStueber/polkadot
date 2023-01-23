// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_tips`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_tips
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `r` is `[0, 16384]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Minimum execution time: 30_721 nanoseconds.
		Weight::from_ref_time(31_898_119)
			// Standard Error: 8
			.saturating_add(Weight::from_ref_time(2_149).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn retract_tip() -> Weight {
		// Minimum execution time: 29_989 nanoseconds.
		Weight::from_ref_time(31_338_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:0 w:1)
	/// The range of component `r` is `[0, 16384]`.
	/// The range of component `t` is `[1, 19]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Minimum execution time: 23_087 nanoseconds.
		Weight::from_ref_time(20_866_394)
			// Standard Error: 9
			.saturating_add(Weight::from_ref_time(1_968).saturating_mul(r.into()))
			// Standard Error: 8_037
			.saturating_add(Weight::from_ref_time(224_436).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `t` is `[1, 19]`.
	fn tip(t: u32, ) -> Weight {
		// Minimum execution time: 14_920 nanoseconds.
		Weight::from_ref_time(15_655_778)
			// Standard Error: 2_069
			.saturating_add(Weight::from_ref_time(131_215).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 19]`.
	fn close_tip(t: u32, ) -> Weight {
		// Minimum execution time: 47_848 nanoseconds.
		Weight::from_ref_time(49_615_292)
			// Standard Error: 7_004
			.saturating_add(Weight::from_ref_time(131_866).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 19]`.
	fn slash_tip(t: u32, ) -> Weight {
		// Minimum execution time: 19_108 nanoseconds.
		Weight::from_ref_time(20_065_253)
			// Standard Error: 2_016
			.saturating_add(Weight::from_ref_time(10_830).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
