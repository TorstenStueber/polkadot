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
//! Autogenerated weights for `pallet_xcm`
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
// --pallet=pallet_xcm
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

/// Weight functions for `pallet_xcm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm::WeightInfo for WeightInfo<T> {
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	fn send() -> Weight {
		// Minimum execution time: 38_016 nanoseconds.
		Weight::from_ref_time(39_046_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn teleport_assets() -> Weight {
		// Minimum execution time: 28_397 nanoseconds.
		Weight::from_ref_time(29_147_000)
	}
	fn reserve_transfer_assets() -> Weight {
		// Minimum execution time: 27_384 nanoseconds.
		Weight::from_ref_time(29_308_000)
	}
	fn execute() -> Weight {
		// Minimum execution time: 15_644 nanoseconds.
		Weight::from_ref_time(16_253_000)
	}
	// Storage: XcmPallet SupportedVersion (r:0 w:1)
	fn force_xcm_version() -> Weight {
		// Minimum execution time: 15_629 nanoseconds.
		Weight::from_ref_time(15_921_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: XcmPallet SafeXcmVersion (r:0 w:1)
	fn force_default_xcm_version() -> Weight {
		// Minimum execution time: 4_171 nanoseconds.
		Weight::from_ref_time(4_349_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: XcmPallet VersionNotifiers (r:1 w:1)
	// Storage: XcmPallet QueryCounter (r:1 w:1)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: XcmPallet Queries (r:0 w:1)
	fn force_subscribe_version_notify() -> Weight {
		// Minimum execution time: 42_081 nanoseconds.
		Weight::from_ref_time(43_216_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: XcmPallet VersionNotifiers (r:1 w:1)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: XcmPallet Queries (r:0 w:1)
	fn force_unsubscribe_version_notify() -> Weight {
		// Minimum execution time: 45_900 nanoseconds.
		Weight::from_ref_time(47_041_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: XcmPallet SupportedVersion (r:4 w:2)
	fn migrate_supported_version() -> Weight {
		// Minimum execution time: 16_233 nanoseconds.
		Weight::from_ref_time(16_828_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: XcmPallet VersionNotifiers (r:4 w:2)
	fn migrate_version_notifiers() -> Weight {
		// Minimum execution time: 16_116 nanoseconds.
		Weight::from_ref_time(16_444_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: XcmPallet VersionNotifyTargets (r:5 w:0)
	fn already_notified_target() -> Weight {
		// Minimum execution time: 19_009 nanoseconds.
		Weight::from_ref_time(19_696_000)
			.saturating_add(T::DbWeight::get().reads(5))
	}
	// Storage: XcmPallet VersionNotifyTargets (r:2 w:1)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	fn notify_current_targets() -> Weight {
		// Minimum execution time: 38_580 nanoseconds.
		Weight::from_ref_time(40_073_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: XcmPallet VersionNotifyTargets (r:3 w:0)
	fn notify_target_migration_fail() -> Weight {
		// Minimum execution time: 8_131 nanoseconds.
		Weight::from_ref_time(8_448_000)
			.saturating_add(T::DbWeight::get().reads(3))
	}
	// Storage: XcmPallet VersionNotifyTargets (r:4 w:2)
	fn migrate_version_notify_targets() -> Weight {
		// Minimum execution time: 16_779 nanoseconds.
		Weight::from_ref_time(17_217_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: XcmPallet VersionNotifyTargets (r:4 w:2)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	fn migrate_and_notify_old_targets() -> Weight {
		// Minimum execution time: 45_150 nanoseconds.
		Weight::from_ref_time(46_055_000)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}
