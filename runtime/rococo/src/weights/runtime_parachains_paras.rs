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
//! Autogenerated weights for `runtime_parachains::paras`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::paras
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/runtime_parachains_paras.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::paras`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras::WeightInfo for WeightInfo<T> {
	// Storage: Paras CurrentCodeHash (r:1 w:1)
	// Storage: Paras CodeByHashRefs (r:1 w:1)
	// Storage: Paras PastCodeMeta (r:1 w:1)
	// Storage: Paras PastCodePruning (r:1 w:1)
	// Storage: Paras PastCodeHash (r:0 w:1)
	// Storage: Paras CodeByHash (r:0 w:1)
	/// The range of component `c` is `[1, 3145728]`.
	fn force_set_current_code(c: u32, ) -> Weight {
		// Minimum execution time: 38_246 nanoseconds.
		Weight::from_ref_time(125_433_017)
			// Standard Error: 12
			.saturating_add(Weight::from_ref_time(2_527).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Paras Heads (r:0 w:1)
	/// The range of component `s` is `[1, 1048576]`.
	fn force_set_current_head(s: u32, ) -> Weight {
		// Minimum execution time: 13_726 nanoseconds.
		Weight::from_ref_time(7_645_690)
			// Standard Error: 2
			.saturating_add(Weight::from_ref_time(1_071).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Paras FutureCodeHash (r:1 w:1)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Paras UpgradeCooldowns (r:1 w:1)
	// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	// Storage: Paras CodeByHash (r:1 w:1)
	// Storage: Paras UpcomingUpgrades (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: Paras CodeByHashRefs (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:0 w:1)
	// Storage: Paras UpgradeRestrictionSignal (r:0 w:1)
	/// The range of component `c` is `[1, 3145728]`.
	fn force_schedule_code_upgrade(c: u32, ) -> Weight {
		// Minimum execution time: 65_770 nanoseconds.
		Weight::from_ref_time(77_383_646)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(2_598).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// The range of component `s` is `[1, 1048576]`.
	fn force_note_new_head(s: u32, ) -> Weight {
		// Minimum execution time: 19_563 nanoseconds.
		Weight::from_ref_time(24_816_837)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_038).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	fn force_queue_action() -> Weight {
		// Minimum execution time: 24_912 nanoseconds.
		Weight::from_ref_time(25_612_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	// Storage: Paras CodeByHash (r:1 w:1)
	/// The range of component `c` is `[1, 3145728]`.
	fn add_trusted_validation_code(c: u32, ) -> Weight {
		// Minimum execution time: 9_248 nanoseconds.
		Weight::from_ref_time(28_464_908)
			// Standard Error: 8
			.saturating_add(Weight::from_ref_time(2_597).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Paras CodeByHashRefs (r:1 w:0)
	// Storage: Paras CodeByHash (r:0 w:1)
	fn poke_unused_validation_code() -> Weight {
		// Minimum execution time: 7_690 nanoseconds.
		Weight::from_ref_time(7_897_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	fn include_pvf_check_statement() -> Weight {
		// Minimum execution time: 124_570 nanoseconds.
		Weight::from_ref_time(131_340_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	// Storage: Paras PvfActiveVoteList (r:1 w:1)
	// Storage: Paras UpcomingUpgrades (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:0 w:100)
	fn include_pvf_check_statement_finalize_upgrade_accept() -> Weight {
		// Minimum execution time: 819_144 nanoseconds.
		Weight::from_ref_time(842_527_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(104))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	fn include_pvf_check_statement_finalize_upgrade_reject() -> Weight {
		// Minimum execution time: 125_093 nanoseconds.
		Weight::from_ref_time(131_146_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	// Storage: Paras PvfActiveVoteList (r:1 w:1)
	// Storage: Paras ActionsQueue (r:1 w:1)
	fn include_pvf_check_statement_finalize_onboarding_accept() -> Weight {
		// Minimum execution time: 646_678 nanoseconds.
		Weight::from_ref_time(661_441_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	fn include_pvf_check_statement_finalize_onboarding_reject() -> Weight {
		// Minimum execution time: 123_755 nanoseconds.
		Weight::from_ref_time(127_740_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
