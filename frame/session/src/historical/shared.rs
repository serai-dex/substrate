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

//! Shared logic between on-chain and off-chain components used for slashing using an off-chain
//! worker.

use codec::Encode;
use sp_staking::SessionIndex;
use sp_std::prelude::*;

pub(super) const PREFIX: &[u8] = b"session_historical";
pub(super) const LAST_PRUNE: &[u8] = b"session_historical_last_prune";

/// Derive the key used to store the list of validators
pub(super) fn derive_key<P: AsRef<[u8]>>(prefix: P, session_index: SessionIndex) -> Vec<u8> {
	session_index.using_encoded(|encoded_session_index| {
		let mut key = prefix.as_ref().to_owned();
		key.push(b'/');
		key.extend_from_slice(encoded_session_index);
		key
	})
}
