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

use crate::{
	traits::{GetStorageVersion, NoStorageVersionSet, PalletInfoAccess, StorageVersion},
	weights::{RuntimeDbWeight, Weight},
};
use impl_trait_for_tuples::impl_for_tuples;
use sp_core::Get;
use sp_io::{hashing::twox_128, storage::clear_prefix, KillStorageResult};
use sp_std::marker::PhantomData;

#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

#[cfg(feature = "experimental")]
use crate::traits::OnRuntimeUpgrade;

/// Make it easier to write versioned runtime upgrades.
///
/// [`VersionedRuntimeUpgrade`] allows developers to write migrations without worrying about
/// checking and setting storage versions. Instead, the developer wraps their migration in this
/// struct which takes care of version handling using best practices.
///
/// It takes 5 type parameters:
/// - `From`: The version being upgraded from.
/// - `To`: The version being upgraded to.
/// - `Inner`: An implementation of `OnRuntimeUpgrade`.
/// - `Pallet`: The Pallet being upgraded.
/// - `Weight`: The runtime's RuntimeDbWeight implementation.
///
/// When a [`VersionedRuntimeUpgrade`] `on_runtime_upgrade`, `pre_upgrade`, or `post_upgrade`
/// method is called, the on-chain version of the pallet is compared to `From`. If they match, the
/// `Inner` equivalent is called and the pallets on-chain version is set to `To` after the
/// migration. Otherwise, a warning is logged notifying the developer that the upgrade was a noop
/// and should probably be removed.
///
/// ### Examples
/// ```ignore
/// // In file defining migrations
/// pub struct VersionUncheckedMigrateV5ToV6<T>(sp_std::marker::PhantomData<T>);
/// impl<T: Config> OnRuntimeUpgrade for VersionUncheckedMigrateV5ToV6<T> {
/// 	// OnRuntimeUpgrade implementation...
/// }
///
/// pub type VersionCheckedMigrateV5ToV6<Runtime, Pallet, DbWeight> =
/// 	VersionedRuntimeUpgrade<5, 6, VersionUncheckedMigrateV5ToV6<Runtime>, Pallet, DbWeight>;
///
/// // Migrations tuple to pass to the Executive pallet:
/// pub type Migrations = (
/// 	// other migrations...
/// 	VersionCheckedMigrateV5ToV6<Runtime, Balances, RuntimeDbWeight>,
/// 	// other migrations...
/// );
/// ```
#[cfg(feature = "experimental")]
pub struct VersionedRuntimeUpgrade<const FROM: u16, const TO: u16, Inner, Pallet, Weight> {
	_marker: PhantomData<(Inner, Pallet, Weight)>,
}

/// A helper enum to wrap the pre_upgrade bytes like an Option before passing them to post_upgrade.
/// This enum is used rather than an Option to make the API clearer to the developer.
#[cfg(feature = "experimental")]
#[derive(codec::Encode, codec::Decode)]
pub enum VersionedPostUpgradeData {
	/// The migration ran, inner vec contains pre_upgrade data.
	MigrationExecuted(Vec<u8>),
	/// This migration is a noop, do not run post_upgrade checks.
	Noop,
}

/// Implementation of the `OnRuntimeUpgrade` trait for `VersionedRuntimeUpgrade`.
///
/// Its main function is to perform the runtime upgrade in `on_runtime_upgrade` only if the on-chain
/// version of the pallets storage matches `From`, and after the upgrade set the on-chain storage to
/// `To`. If the versions do not match, it writes a log notifying the developer that the migration
/// is a noop.
#[cfg(feature = "experimental")]
impl<
		const FROM: u16,
		const TO: u16,
		Inner: OnRuntimeUpgrade,
		Pallet: GetStorageVersion<CurrentStorageVersion = StorageVersion> + PalletInfoAccess,
		DbWeight: Get<RuntimeDbWeight>,
	> OnRuntimeUpgrade for VersionedRuntimeUpgrade<FROM, TO, Inner, Pallet, DbWeight>
{
	/// Executes pre_upgrade if the migration will run, and wraps the pre_upgrade bytes in
	/// [`VersionedPostUpgradeData`] before passing them to post_upgrade, so it knows whether the
	/// migration ran or not.
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
		use codec::Encode;
		let on_chain_version = Pallet::on_chain_storage_version();
		if on_chain_version == FROM {
			Ok(VersionedPostUpgradeData::MigrationExecuted(Inner::pre_upgrade()?).encode())
		} else {
			Ok(VersionedPostUpgradeData::Noop.encode())
		}
	}

	/// Executes the versioned runtime upgrade.
	///
	/// First checks if the pallets on-chain storage version matches the version of this upgrade. If
	/// it matches, it calls `Inner::on_runtime_upgrade`, updates the on-chain version, and returns
	/// the weight. If it does not match, it writes a log notifying the developer that the migration
	/// is a noop.
	fn on_runtime_upgrade() -> Weight {
		let on_chain_version = Pallet::on_chain_storage_version();
		if on_chain_version == FROM {
			log::info!(
				"Running {} VersionedOnRuntimeUpgrade: version {:?} to {:?}.",
				Pallet::name(),
				FROM,
				TO
			);

			// Execute the migration
			let weight = Inner::on_runtime_upgrade();

			// Update the on-chain version
			StorageVersion::new(TO).put::<Pallet>();

			weight.saturating_add(DbWeight::get().reads_writes(1, 1))
		} else {
			log::warn!(
				"{} VersionedOnRuntimeUpgrade for version {:?} skipped because current on-chain version is {:?}.",
				Pallet::name(),
				FROM,
				on_chain_version
			);
			DbWeight::get().reads(1)
		}
	}

	/// Executes `Inner::post_upgrade` if the migration just ran.
	///
	/// pre_upgrade passes [`VersionedPostUpgradeData::MigrationExecuted`] to post_upgrade if
	/// the migration ran, and [`VersionedPostUpgradeData::Noop`] otherwise.
	#[cfg(feature = "try-runtime")]
	fn post_upgrade(
		versioned_post_upgrade_data_bytes: Vec<u8>,
	) -> Result<(), sp_runtime::TryRuntimeError> {
		use codec::DecodeAll;
		match <VersionedPostUpgradeData>::decode_all(&mut &versioned_post_upgrade_data_bytes[..])
			.map_err(|_| "VersionedRuntimeUpgrade post_upgrade failed to decode PreUpgradeData")?
		{
			VersionedPostUpgradeData::MigrationExecuted(inner_bytes) =>
				Inner::post_upgrade(inner_bytes),
			VersionedPostUpgradeData::Noop => Ok(()),
		}
	}
}

/// Can store the current pallet version in storage.
pub trait StoreCurrentStorageVersion<T: GetStorageVersion + PalletInfoAccess> {
	/// Write the current storage version to the storage.
	fn store_current_storage_version();
}

impl<T: GetStorageVersion<CurrentStorageVersion = StorageVersion> + PalletInfoAccess>
	StoreCurrentStorageVersion<T> for StorageVersion
{
	fn store_current_storage_version() {
		let version = <T as GetStorageVersion>::current_storage_version();
		version.put::<T>();
	}
}

impl<T: GetStorageVersion<CurrentStorageVersion = NoStorageVersionSet> + PalletInfoAccess>
	StoreCurrentStorageVersion<T> for NoStorageVersionSet
{
	fn store_current_storage_version() {
		StorageVersion::default().put::<T>();
	}
}

/// Trait used by [`migrate_from_pallet_version_to_storage_version`] to do the actual migration.
pub trait PalletVersionToStorageVersionHelper {
	fn migrate(db_weight: &RuntimeDbWeight) -> Weight;
}

impl<T: GetStorageVersion + PalletInfoAccess> PalletVersionToStorageVersionHelper for T
where
	T::CurrentStorageVersion: StoreCurrentStorageVersion<T>,
{
	fn migrate(db_weight: &RuntimeDbWeight) -> Weight {
		const PALLET_VERSION_STORAGE_KEY_POSTFIX: &[u8] = b":__PALLET_VERSION__:";

		fn pallet_version_key(name: &str) -> [u8; 32] {
			crate::storage::storage_prefix(name.as_bytes(), PALLET_VERSION_STORAGE_KEY_POSTFIX)
		}

		sp_io::storage::clear(&pallet_version_key(<T as PalletInfoAccess>::name()));

		<T::CurrentStorageVersion as StoreCurrentStorageVersion<T>>::store_current_storage_version(
		);

		db_weight.writes(2)
	}
}

#[cfg_attr(all(not(feature = "tuples-96"), not(feature = "tuples-128")), impl_for_tuples(64))]
#[cfg_attr(all(feature = "tuples-96", not(feature = "tuples-128")), impl_for_tuples(96))]
#[cfg_attr(feature = "tuples-128", impl_for_tuples(128))]
impl PalletVersionToStorageVersionHelper for T {
	fn migrate(db_weight: &RuntimeDbWeight) -> Weight {
		let mut weight = Weight::zero();

		for_tuples!( #( weight = weight.saturating_add(T::migrate(db_weight)); )* );

		weight
	}
}

/// Migrate from the `PalletVersion` struct to the new
/// [`StorageVersion`](crate::traits::StorageVersion) struct.
///
/// This will remove all `PalletVersion's` from the state and insert the current storage version.
pub fn migrate_from_pallet_version_to_storage_version<
	Pallets: PalletVersionToStorageVersionHelper,
>(
	db_weight: &RuntimeDbWeight,
) -> Weight {
	Pallets::migrate(db_weight)
}

/// `RemovePallet` is a utility struct used to remove all storage items associated with a specific
/// pallet.
///
/// This struct is generic over two parameters:
/// - `P` is a type that implements the `Get` trait for a static string, representing the pallet's
///   name.
/// - `DbWeight` is a type that implements the `Get` trait for `RuntimeDbWeight`, providing the
///   weight for database operations.
///
/// On runtime upgrade, the `on_runtime_upgrade` function will clear all storage items associated
/// with the specified pallet, logging the number of keys removed. If the `try-runtime` feature is
/// enabled, the `pre_upgrade` and `post_upgrade` functions can be used to verify the storage
/// removal before and after the upgrade.
///
/// # Examples:
/// ```ignore
/// construct_runtime! {
/// 	pub enum Runtime
/// 	{
/// 		System: frame_system::{Pallet, Call, Storage, Config<T>, Event<T>} = 0,
///
/// 		SomePalletToRemove: pallet_something::{Pallet, Call, Storage, Event<T>} = 1,
/// 		AnotherPalletToRemove: pallet_something_else::{Pallet, Call, Storage, Event<T>} = 2,
///
/// 		YourOtherPallets...
/// 	}
/// };
///
/// parameter_types! {
/// 		pub const SomePalletToRemoveStr: &'static str = "SomePalletToRemove";
/// 		pub const AnotherPalletToRemoveStr: &'static str = "AnotherPalletToRemove";
/// }
///
/// pub type Migrations = (
/// 	RemovePallet<SomePalletToRemoveStr, RocksDbWeight>,
/// 	RemovePallet<AnotherPalletToRemoveStr, RocksDbWeight>,
/// 	AnyOtherMigrations...
/// );
///
/// pub type Executive = frame_executive::Executive<
/// 	Runtime,
/// 	Block,
/// 	frame_system::ChainContext<Runtime>,
/// 	Runtime,
/// 	Migrations
/// >;
/// ```
///
/// WARNING: `RemovePallet` has no guard rails preventing it from bricking the chain if the
/// operation of removing storage for the given pallet would exceed the block weight limit.
///
/// If your pallet has too many keys to be removed in a single block, it is advised to wait for
/// a multi-block scheduler currently under development which will allow for removal of storage
/// items (and performing other heavy migrations) over multiple blocks
/// (see <https://github.com/paritytech/substrate/issues/13690>).
pub struct RemovePallet<P: Get<&'static str>, DbWeight: Get<RuntimeDbWeight>>(
	PhantomData<(P, DbWeight)>,
);
impl<P: Get<&'static str>, DbWeight: Get<RuntimeDbWeight>> frame_support::traits::OnRuntimeUpgrade
	for RemovePallet<P, DbWeight>
{
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		let hashed_prefix = twox_128(P::get().as_bytes());
		let keys_removed = match clear_prefix(&hashed_prefix, None) {
			KillStorageResult::AllRemoved(value) => value,
			KillStorageResult::SomeRemaining(value) => {
				log::error!(
					"`clear_prefix` failed to remove all keys for {}. THIS SHOULD NEVER HAPPEN! 🚨",
					P::get()
				);
				value
			},
		} as u64;

		log::info!("Removed {} {} keys 🧹", keys_removed, P::get());

		DbWeight::get().reads_writes(keys_removed + 1, keys_removed)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
		use crate::storage::unhashed::contains_prefixed_key;

		let hashed_prefix = twox_128(P::get().as_bytes());
		match contains_prefixed_key(&hashed_prefix) {
			true => log::info!("Found {} keys pre-removal 👀", P::get()),
			false => log::warn!(
				"Migration RemovePallet<{}> can be removed (no keys found pre-removal).",
				P::get()
			),
		};
		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
		use crate::storage::unhashed::contains_prefixed_key;

		let hashed_prefix = twox_128(P::get().as_bytes());
		match contains_prefixed_key(&hashed_prefix) {
			true => {
				log::error!("{} has keys remaining post-removal ❗", P::get());
				return Err("Keys remaining post-removal, this should never happen 🚨".into())
			},
			false => log::info!("No {} keys found post-removal 🎉", P::get()),
		};
		Ok(())
	}
}
