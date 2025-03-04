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

//! Utilities to build a `TestClient` for `kitchensink-runtime`.

use sp_runtime::BuildStorage;
/// Re-export test-client utilities.
pub use substrate_test_client::*;

/// Call executor for `kitchensink-runtime` `TestClient`.
pub type ExecutorDispatch = sc_executor::NativeElseWasmExecutor<node_executor::ExecutorDispatch>;

/// Default backend type.
pub type Backend = sc_client_db::Backend<node_primitives::Block>;

/// Test client type.
pub type Client = client::Client<
	Backend,
	client::LocalCallExecutor<node_primitives::Block, Backend, ExecutorDispatch>,
	node_primitives::Block,
	kitchensink_runtime::RuntimeApi,
>;

/// Transaction for kitchensink-runtime.
pub type Transaction = sc_client_api::backend::TransactionFor<Backend, node_primitives::Block>;

/// Genesis configuration parameters for `TestClient`.
#[derive(Default)]
pub struct GenesisParameters;

impl substrate_test_client::GenesisInit for GenesisParameters {
	fn genesis_storage(&self) -> Storage {
		crate::genesis::config(None).build_storage().unwrap()
	}
}

/// A `test-runtime` extensions to `TestClientBuilder`.
pub trait TestClientBuilderExt: Sized {
	/// Create test client builder.
	fn new() -> Self;

	/// Build the test client.
	fn build(self) -> Client;
}

impl TestClientBuilderExt
	for substrate_test_client::TestClientBuilder<
		node_primitives::Block,
		client::LocalCallExecutor<node_primitives::Block, Backend, ExecutorDispatch>,
		Backend,
		GenesisParameters,
	>
{
	fn new() -> Self {
		Self::default()
	}

	fn build(self) -> Client {
		self.build_with_native_executor(None).0
	}
}
