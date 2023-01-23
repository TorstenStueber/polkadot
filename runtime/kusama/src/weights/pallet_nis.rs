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
//! Autogenerated weights for `pallet_nis`
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
// --pallet=pallet_nis
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

/// Weight functions for `pallet_nis`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nis::WeightInfo for WeightInfo<T> {
	// Storage: Nis Queues (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: Nis QueueTotals (r:1 w:1)
	/// The range of component `l` is `[0, 999]`.
	fn place_bid(l: u32, ) -> Weight {
		// Minimum execution time: 37_780 nanoseconds.
		Weight::from_ref_time(37_851_979)
			// Standard Error: 1_271
			.saturating_add(Weight::from_ref_time(99_606).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Nis Queues (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: Nis QueueTotals (r:1 w:1)
	fn place_bid_max() -> Weight {
		// Minimum execution time: 131_727 nanoseconds.
		Weight::from_ref_time(135_467_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Nis Queues (r:1 w:1)
	// Storage: Nis QueueTotals (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	/// The range of component `l` is `[1, 1000]`.
	fn retract_bid(l: u32, ) -> Weight {
		// Minimum execution time: 46_264 nanoseconds.
		Weight::from_ref_time(39_987_438)
			// Standard Error: 1_185
			.saturating_add(Weight::from_ref_time(82_927).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Nis Summary (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn fund_deficit() -> Weight {
		// Minimum execution time: 43_743 nanoseconds.
		Weight::from_ref_time(45_150_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Nis Receipts (r:1 w:1)
	// Storage: Nis Summary (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	fn thaw_private() -> Weight {
		// Minimum execution time: 57_646 nanoseconds.
		Weight::from_ref_time(58_259_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Nis Receipts (r:1 w:1)
	// Storage: Nis Summary (r:1 w:1)
	// Storage: NisCounterpartBalances Account (r:1 w:1)
	// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn thaw_communal() -> Weight {
		// Minimum execution time: 73_710 nanoseconds.
		Weight::from_ref_time(75_420_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Nis Receipts (r:1 w:1)
	// Storage: Nis Summary (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: NisCounterpartBalances Account (r:1 w:1)
	// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	fn privatize() -> Weight {
		// Minimum execution time: 80_694 nanoseconds.
		Weight::from_ref_time(83_892_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Nis Receipts (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Nis Summary (r:1 w:1)
	// Storage: NisCounterpartBalances Account (r:1 w:1)
	// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	fn communify() -> Weight {
		// Minimum execution time: 70_940 nanoseconds.
		Weight::from_ref_time(73_876_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Nis Summary (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Nis QueueTotals (r:1 w:1)
	fn process_queues() -> Weight {
		// Minimum execution time: 50_584 nanoseconds.
		Weight::from_ref_time(55_042_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Nis Queues (r:1 w:1)
	fn process_queue() -> Weight {
		// Minimum execution time: 4_445 nanoseconds.
		Weight::from_ref_time(4_588_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Nis Receipts (r:0 w:1)
	fn process_bid() -> Weight {
		// Minimum execution time: 11_503 nanoseconds.
		Weight::from_ref_time(11_903_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
