// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_election_provider_support_benchmarking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Rosss-MacBook-Pro-2.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/substrate
// benchmark
// pallet
// --execution
// wasm
// --wasm-execution
// compiled
// --dev
// --pallet
// pallet-election-provider-support-benchmarking
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// frame/election-provider-support/src/weights.rs
// --template
// .maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_election_provider_support_benchmarking.
pub trait WeightInfo {
	fn phragmen(v: u32, t: u32, d: u32, ) -> Weight;
	fn phragmms(v: u32, t: u32, d: u32, ) -> Weight;
	fn approval_voting(v: u32, t: u32, d: u32, ) -> Weight;
}

/// Weights for pallet_election_provider_support_benchmarking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `d` is `[5, 16]`.
	fn phragmen(v: u32, _t: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_215_000 nanoseconds.
		Weight::from_ref_time(5_325_000_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 106_262
			.saturating_add(Weight::from_ref_time(3_989_100).saturating_mul(v.into()))
			// Standard Error: 10_863_902
			.saturating_add(Weight::from_ref_time(1_008_030_786).saturating_mul(d.into()))
	}
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `d` is `[5, 16]`.
	fn phragmms(v: u32, _t: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_703_000 nanoseconds.
		Weight::from_ref_time(3_715_000_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 85_871
			.saturating_add(Weight::from_ref_time(3_443_928).saturating_mul(v.into()))
			// Standard Error: 8_779_237
			.saturating_add(Weight::from_ref_time(985_037_659).saturating_mul(d.into()))
	}
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `d` is `[5, 16]`.
	fn approval_voting(v: u32, _t: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_863_000 nanoseconds.
		Weight::from_ref_time(1_868_000_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 24_930
			.saturating_add(Weight::from_ref_time(1_215_357).saturating_mul(v.into()))
			// Standard Error: 2_548_811
			.saturating_add(Weight::from_ref_time(197_537_394).saturating_mul(d.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `d` is `[5, 16]`.
	fn phragmen(v: u32, _t: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_215_000 nanoseconds.
		Weight::from_ref_time(5_325_000_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 106_262
			.saturating_add(Weight::from_ref_time(3_989_100).saturating_mul(v.into()))
			// Standard Error: 10_863_902
			.saturating_add(Weight::from_ref_time(1_008_030_786).saturating_mul(d.into()))
	}
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `d` is `[5, 16]`.
	fn phragmms(v: u32, _t: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_703_000 nanoseconds.
		Weight::from_ref_time(3_715_000_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 85_871
			.saturating_add(Weight::from_ref_time(3_443_928).saturating_mul(v.into()))
			// Standard Error: 8_779_237
			.saturating_add(Weight::from_ref_time(985_037_659).saturating_mul(d.into()))
	}
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `d` is `[5, 16]`.
	fn approval_voting(v: u32, _t: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_863_000 nanoseconds.
		Weight::from_ref_time(1_868_000_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 24_930
			.saturating_add(Weight::from_ref_time(1_215_357).saturating_mul(v.into()))
			// Standard Error: 2_548_811
			.saturating_add(Weight::from_ref_time(197_537_394).saturating_mul(d.into()))
	}
}
