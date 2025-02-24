// This file is part of a fork of Substrate which has had various changes.

// Copyright (C) Parity Technologies (UK) Ltd.
// Copyright (C) 2022-2023 Luke Parker
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use frame_support::derive_impl;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_core::{sr25519, ConstU64};
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, Verify},
};

#[frame_support::pallet]
mod module {
	use super::*;
	use frame_support::pallet_prelude::*;

	pub type Request<T> = (<T as frame_system::Config>::AccountId, Role, BlockNumberFor<T>);
	pub type Requests<T> = Vec<Request<T>>;

	#[derive(Copy, Clone, Eq, PartialEq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum Role {
		Storage,
	}

	#[derive(Copy, Clone, Eq, PartialEq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct RoleParameters<T: Config> {
		// minimum actors to maintain - if role is unstaking
		// and remaining actors would be less that this value - prevent or punish for unstaking
		pub min_actors: u32,

		// the maximum number of spots available to fill for a role
		pub max_actors: u32,

		// payouts are made at this block interval
		pub reward_period: BlockNumberFor<T>,

		// minimum amount of time before being able to unstake
		pub bonding_period: BlockNumberFor<T>,

		// how long tokens remain locked for after unstaking
		pub unbonding_period: BlockNumberFor<T>,

		// minimum period required to be in service. unbonding before this time is highly penalized
		pub min_service_period: BlockNumberFor<T>,

		// "startup" time allowed for roles that need to sync their infrastructure
		// with other providers before they are considered in service and punishable for
		// not delivering required level of service.
		pub startup_grace_period: BlockNumberFor<T>,
	}

	impl<T: Config> Default for RoleParameters<T> {
		fn default() -> Self {
			Self {
				max_actors: 10,
				reward_period: BlockNumberFor::<T>::default(),
				unbonding_period: BlockNumberFor::<T>::default(),

				// not currently used
				min_actors: 5,
				bonding_period: BlockNumberFor::<T>::default(),
				min_service_period: BlockNumberFor::<T>::default(),
				startup_grace_period: BlockNumberFor::<T>::default(),
			}
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + TypeInfo {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	/// requirements to enter and maintain status in roles
	#[pallet::storage]
	#[pallet::getter(fn parameters)]
	pub type Parameters<T: Config> =
		StorageMap<_, Blake2_128Concat, Role, RoleParameters<T>, OptionQuery>;

	/// the roles members can enter into
	#[pallet::storage]
	#[pallet::getter(fn available_roles)]
	#[pallet::unbounded]
	pub type AvailableRoles<T: Config> = StorageValue<_, Vec<Role>, ValueQuery>;

	/// Actors list
	#[pallet::storage]
	#[pallet::getter(fn actor_account_ids)]
	#[pallet::unbounded]
	pub type ActorAccountIds<T: Config> = StorageValue<_, Vec<T::AccountId>>;

	/// actor accounts associated with a role
	#[pallet::storage]
	#[pallet::getter(fn account_ids_by_role)]
	#[pallet::unbounded]
	pub type AccountIdsByRole<T: Config> = StorageMap<_, Blake2_128Concat, Role, Vec<T::AccountId>>;

	/// tokens locked until given block number
	#[pallet::storage]
	#[pallet::getter(fn bondage)]
	pub type Bondage<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BlockNumberFor<T>>;

	/// First step before enter a role is registering intent with a new account/key.
	/// This is done by sending a role_entry_request() from the new account.
	/// The member must then send a stake() transaction to approve the request and enter the desired
	/// role. The account making the request will be bonded and must have
	/// sufficient balance to cover the minimum stake for the role.
	/// Bonding only occurs after successful entry into a role.
	#[pallet::storage]
	#[pallet::getter(fn role_entry_requests)]
	#[pallet::unbounded]
	pub type RoleEntryRequests<T: Config> = StorageValue<_, Requests<T>>;

	/// Entry request expires after this number of blocks
	#[pallet::storage]
	#[pallet::getter(fn request_life_time)]
	pub type RequestLifeTime<T: Config> = StorageValue<_, u64, ValueQuery, ConstU64<0>>;

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub enable_storage_role: bool,
		pub request_life_time: u64,
		#[serde(skip)]
		pub _config: sp_std::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			if self.enable_storage_role {
				<Parameters<T>>::insert(Role::Storage, <RoleParameters<T>>::default());
				<AvailableRoles<T>>::put(vec![Role::Storage]);
			}
			<RequestLifeTime<T>>::put(self.request_life_time);
		}
	}
}

pub type BlockNumber = u64;
pub type Signature = sr25519::Signature;
pub type AccountId = <Signature as Verify>::Signer;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<u32, RuntimeCall, Signature, ()>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type Block = Block;
	type BlockHashCount = ConstU64<10>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type PalletInfo = PalletInfo;
	type OnSetCode = ();
}

impl module::Config for Runtime {}

frame_support::construct_runtime!(
	pub struct Runtime {
		System: frame_system,
		Module: module,
	}
);

#[test]
fn create_genesis_config() {
	let config = RuntimeGenesisConfig {
		system: Default::default(),
		module: module::GenesisConfig {
			request_life_time: 0,
			enable_storage_role: true,
			..Default::default()
		},
	};
	assert_eq!(config.module.request_life_time, 0);
	assert!(config.module.enable_storage_role);
}
