// This file is part of a fork of Substrate which has had various changes.

// Copyright (C) Parity Technologies (UK) Ltd.
// Copyright (C) 2022-2023 Luke Parker
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Genesis Configuration.

use crate::keyring::*;
use kitchensink_runtime::{
	constants::currency::*, wasm_binary_unwrap, AccountId, AssetsConfig, BabeConfig,
	BalancesConfig, GluttonConfig, GrandpaConfig, RuntimeGenesisConfig, SessionConfig,
	StakerStatus, StakingConfig, SystemConfig, BABE_GENESIS_EPOCH_CONFIG,
};
use sp_keyring::Sr25519Keyring;
use sp_runtime::Perbill;

/// Create genesis runtime configuration for tests.
pub fn config(code: Option<&[u8]>) -> RuntimeGenesisConfig {
	config_endowed(code, Default::default())
}

/// Create genesis runtime configuration for tests with some extra
/// endowed accounts.
pub fn config_endowed(code: Option<&[u8]>, extra_endowed: Vec<AccountId>) -> RuntimeGenesisConfig {
	let mut endowed = vec![
		(alice(), 111 * DOLLARS),
		(bob(), 100 * DOLLARS),
		(charlie(), 100_000_000 * DOLLARS),
		(dave(), 111 * DOLLARS),
		(eve(), 101 * DOLLARS),
		(ferdie(), 100 * DOLLARS),
	];

	endowed.extend(extra_endowed.into_iter().map(|endowed| (endowed, 100 * DOLLARS)));

	RuntimeGenesisConfig {
		system: SystemConfig {
			code: code.map(|x| x.to_vec()).unwrap_or_else(|| wasm_binary_unwrap().to_vec()),
			..Default::default()
		},
		balances: BalancesConfig { balances: endowed },
		session: SessionConfig {
			keys: vec![
				(alice(), dave(), to_session_keys(&Sr25519Keyring::Alice)),
				(bob(), eve(), to_session_keys(&Sr25519Keyring::Bob)),
				(charlie(), ferdie(), to_session_keys(&Sr25519Keyring::Charlie)),
			],
		},
		staking: StakingConfig {
			stakers: vec![
				(dave(), dave(), 111 * DOLLARS, StakerStatus::Validator),
				(eve(), eve(), 100 * DOLLARS, StakerStatus::Validator),
				(ferdie(), ferdie(), 100 * DOLLARS, StakerStatus::Validator),
			],
			validator_count: 3,
			minimum_validator_count: 0,
			slash_reward_fraction: Perbill::from_percent(10),
			invulnerables: vec![alice(), bob(), charlie()],
			..Default::default()
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		grandpa: GrandpaConfig { authorities: vec![], _config: Default::default() },
		im_online: Default::default(),
		authority_discovery: Default::default(),
		sudo: Default::default(),
		assets: AssetsConfig { assets: vec![(9, alice(), true, 1)], ..Default::default() },
		pool_assets: Default::default(),
		transaction_payment: Default::default(),
		glutton: GluttonConfig {
			compute: Default::default(),
			storage: Default::default(),
			trash_data_count: Default::default(),
			..Default::default()
		},
	}
}
