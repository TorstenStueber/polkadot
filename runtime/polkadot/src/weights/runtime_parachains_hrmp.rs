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
//! Autogenerated weights for `runtime_parachains::hrmp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::hrmp
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/runtime_parachains_hrmp.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::hrmp`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::hrmp::WeightInfo for WeightInfo<T> {
	// Storage: Paras ParaLifecycles (r:2 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpChannels (r:1 w:0)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	fn hrmp_init_open_channel() -> Weight {
		// Minimum execution time: 43_278 nanoseconds.
		Weight::from_ref_time(44_345_000)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Paras ParaLifecycles (r:1 w:0)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:1 w:0)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	fn hrmp_accept_open_channel() -> Weight {
		// Minimum execution time: 42_598 nanoseconds.
		Weight::from_ref_time(43_414_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Hrmp HrmpChannels (r:1 w:0)
	// Storage: Hrmp HrmpCloseChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	fn hrmp_close_channel() -> Weight {
		// Minimum execution time: 38_720 nanoseconds.
		Weight::from_ref_time(39_534_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Hrmp HrmpIngressChannelsIndex (r:128 w:127)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:1)
	// Storage: Hrmp HrmpChannels (r:127 w:127)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:0 w:1)
	// Storage: Hrmp HrmpChannelContents (r:0 w:127)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:0 w:1)
	/// The range of component `i` is `[0, 127]`.
	/// The range of component `e` is `[0, 127]`.
	fn force_clean_hrmp(i: u32, e: u32, ) -> Weight {
		// Minimum execution time: 1_038_557 nanoseconds.
		Weight::from_ref_time(1_052_333_000)
			// Standard Error: 96_414
			.saturating_add(Weight::from_ref_time(3_228_714).saturating_mul(i.into()))
			// Standard Error: 96_414
			.saturating_add(Weight::from_ref_time(3_264_256).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(e.into())))
	}
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequests (r:2 w:2)
	// Storage: Paras ParaLifecycles (r:4 w:0)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:2 w:2)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:2 w:2)
	// Storage: Hrmp HrmpChannels (r:0 w:2)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_open(c: u32, ) -> Weight {
		// Minimum execution time: 9_936 nanoseconds.
		Weight::from_ref_time(10_543_126)
			// Standard Error: 16_207
			.saturating_add(Weight::from_ref_time(18_523_575).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(c.into())))
	}
	// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:0)
	// Storage: Hrmp HrmpChannels (r:2 w:2)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpCloseChannelRequests (r:0 w:2)
	// Storage: Hrmp HrmpChannelContents (r:0 w:2)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_close(c: u32, ) -> Weight {
		// Minimum execution time: 6_807 nanoseconds.
		Weight::from_ref_time(2_928_678)
			// Standard Error: 13_610
			.saturating_add(Weight::from_ref_time(11_428_924).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(c.into())))
	}
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	/// The range of component `c` is `[0, 128]`.
	fn hrmp_cancel_open_request(c: u32, ) -> Weight {
		// Minimum execution time: 25_994 nanoseconds.
		Weight::from_ref_time(35_768_473)
			// Standard Error: 3_041
			.saturating_add(Weight::from_ref_time(195_812).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequests (r:2 w:2)
	/// The range of component `c` is `[0, 128]`.
	fn clean_open_channel_requests(c: u32, ) -> Weight {
		// Minimum execution time: 4_914 nanoseconds.
		Weight::from_ref_time(1_167_892)
			// Standard Error: 4_929
			.saturating_add(Weight::from_ref_time(3_222_356).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
	// Storage: Paras ParaLifecycles (r:2 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpChannels (r:1 w:0)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:2 w:2)
	// Storage: Dmp DownwardMessageQueueHeads (r:2 w:2)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:1 w:0)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:1 w:1)
	fn force_open_hrmp_channel() -> Weight {
		// Minimum execution time: 55_575 nanoseconds.
		Weight::from_ref_time(57_798_000)
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(8))
	}
}
