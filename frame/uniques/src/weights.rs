// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_uniques
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `test-bench-bot`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/substrate
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_uniques
// --chain=dev
// --output=./frame/uniques/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_uniques.
pub trait WeightInfo {
	fn create() -> Weight;
	fn force_create() -> Weight;
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight;
	fn mint() -> Weight;
	fn burn() -> Weight;
	fn transfer() -> Weight;
	fn redeposit(i: u32, ) -> Weight;
	fn freeze() -> Weight;
	fn thaw() -> Weight;
	fn freeze_collection() -> Weight;
	fn thaw_collection() -> Weight;
	fn transfer_ownership() -> Weight;
	fn set_team() -> Weight;
	fn force_item_status() -> Weight;
	fn set_attribute() -> Weight;
	fn clear_attribute() -> Weight;
	fn set_metadata() -> Weight;
	fn clear_metadata() -> Weight;
	fn set_collection_metadata() -> Weight;
	fn clear_collection_metadata() -> Weight;
	fn approve_transfer() -> Weight;
	fn cancel_approval() -> Weight;
	fn set_accept_ownership() -> Weight;
	fn set_collection_max_supply() -> Weight;
	fn set_price() -> Weight;
	fn buy_item() -> Weight;
}

/// Weights for pallet_uniques using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn create() -> Weight {
		Weight::from_ref_time(33_075_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_create() -> Weight {
		Weight::from_ref_time(19_528_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques Attribute (r:0 w:1000)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Storage: Uniques InstanceMetadataOf (r:0 w:1000)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	// Storage: Uniques Account (r:0 w:20)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		Weight::from_ref_time(0 as RefTimeWeight)
			// Standard Error: 25_000
			.saturating_add(Weight::from_ref_time(13_639_000 as RefTimeWeight).scalar_saturating_mul(n as RefTimeWeight))
			// Standard Error: 25_000
			.saturating_add(Weight::from_ref_time(2_393_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			// Standard Error: 25_000
			.saturating_add(Weight::from_ref_time(2_217_000 as RefTimeWeight).scalar_saturating_mul(a as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(n as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((2 as RefTimeWeight).saturating_mul(n as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(m as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(a as RefTimeWeight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques Account (r:0 w:1)
	fn mint() -> Weight {
		Weight::from_ref_time(42_146_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn burn() -> Weight {
		Weight::from_ref_time(42_960_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:2)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn transfer() -> Weight {
		Weight::from_ref_time(33_025_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:100 w:100)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		Weight::from_ref_time(0 as RefTimeWeight)
			// Standard Error: 24_000
			.saturating_add(Weight::from_ref_time(15_540_000 as RefTimeWeight).scalar_saturating_mul(i as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(i as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(i as RefTimeWeight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn freeze() -> Weight {
		Weight::from_ref_time(25_194_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn thaw() -> Weight {
		Weight::from_ref_time(25_397_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn freeze_collection() -> Weight {
		Weight::from_ref_time(19_278_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn thaw_collection() -> Weight {
		Weight::from_ref_time(19_304_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:2)
	fn transfer_ownership() -> Weight {
		Weight::from_ref_time(28_615_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn set_team() -> Weight {
		Weight::from_ref_time(19_943_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_item_status() -> Weight {
		Weight::from_ref_time(22_583_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn set_attribute() -> Weight {
		Weight::from_ref_time(47_520_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn clear_attribute() -> Weight {
		Weight::from_ref_time(45_316_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn set_metadata() -> Weight {
		Weight::from_ref_time(38_391_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn clear_metadata() -> Weight {
		Weight::from_ref_time(38_023_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn set_collection_metadata() -> Weight {
		Weight::from_ref_time(37_398_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn clear_collection_metadata() -> Weight {
		Weight::from_ref_time(35_621_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn approve_transfer() -> Weight {
		Weight::from_ref_time(25_856_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn cancel_approval() -> Weight {
		Weight::from_ref_time(26_098_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	fn set_accept_ownership() -> Weight {
		Weight::from_ref_time(24_076_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn set_collection_max_supply() -> Weight {
		Weight::from_ref_time(22_035_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn set_price() -> Weight {
		Weight::from_ref_time(22_534_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques ItemPriceOf (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Account (r:0 w:2)
	fn buy_item() -> Weight {
		Weight::from_ref_time(45_272_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn create() -> Weight {
		Weight::from_ref_time(33_075_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_create() -> Weight {
		Weight::from_ref_time(19_528_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques Attribute (r:0 w:1000)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Storage: Uniques InstanceMetadataOf (r:0 w:1000)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	// Storage: Uniques Account (r:0 w:20)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		Weight::from_ref_time(0 as RefTimeWeight)
			// Standard Error: 25_000
			.saturating_add(Weight::from_ref_time(13_639_000 as RefTimeWeight).scalar_saturating_mul(n as RefTimeWeight))
			// Standard Error: 25_000
			.saturating_add(Weight::from_ref_time(2_393_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			// Standard Error: 25_000
			.saturating_add(Weight::from_ref_time(2_217_000 as RefTimeWeight).scalar_saturating_mul(a as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads((1 as RefTimeWeight).saturating_mul(n as RefTimeWeight)))
			.saturating_add(RocksDbWeight::get().writes(4 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes((2 as RefTimeWeight).saturating_mul(n as RefTimeWeight)))
			.saturating_add(RocksDbWeight::get().writes((1 as RefTimeWeight).saturating_mul(m as RefTimeWeight)))
			.saturating_add(RocksDbWeight::get().writes((1 as RefTimeWeight).saturating_mul(a as RefTimeWeight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques Account (r:0 w:1)
	fn mint() -> Weight {
		Weight::from_ref_time(42_146_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn burn() -> Weight {
		Weight::from_ref_time(42_960_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:2)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn transfer() -> Weight {
		Weight::from_ref_time(33_025_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:100 w:100)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		Weight::from_ref_time(0 as RefTimeWeight)
			// Standard Error: 24_000
			.saturating_add(Weight::from_ref_time(15_540_000 as RefTimeWeight).scalar_saturating_mul(i as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads((1 as RefTimeWeight).saturating_mul(i as RefTimeWeight)))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes((1 as RefTimeWeight).saturating_mul(i as RefTimeWeight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn freeze() -> Weight {
		Weight::from_ref_time(25_194_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn thaw() -> Weight {
		Weight::from_ref_time(25_397_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn freeze_collection() -> Weight {
		Weight::from_ref_time(19_278_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn thaw_collection() -> Weight {
		Weight::from_ref_time(19_304_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:2)
	fn transfer_ownership() -> Weight {
		Weight::from_ref_time(28_615_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn set_team() -> Weight {
		Weight::from_ref_time(19_943_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_item_status() -> Weight {
		Weight::from_ref_time(22_583_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn set_attribute() -> Weight {
		Weight::from_ref_time(47_520_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn clear_attribute() -> Weight {
		Weight::from_ref_time(45_316_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn set_metadata() -> Weight {
		Weight::from_ref_time(38_391_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn clear_metadata() -> Weight {
		Weight::from_ref_time(38_023_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn set_collection_metadata() -> Weight {
		Weight::from_ref_time(37_398_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn clear_collection_metadata() -> Weight {
		Weight::from_ref_time(35_621_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn approve_transfer() -> Weight {
		Weight::from_ref_time(25_856_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn cancel_approval() -> Weight {
		Weight::from_ref_time(26_098_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	fn set_accept_ownership() -> Weight {
		Weight::from_ref_time(24_076_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn set_collection_max_supply() -> Weight {
		Weight::from_ref_time(22_035_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn set_price() -> Weight {
		Weight::from_ref_time(22_534_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques ItemPriceOf (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Account (r:0 w:2)
	fn buy_item() -> Weight {
		Weight::from_ref_time(45_272_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(4 as RefTimeWeight))
	}
}
