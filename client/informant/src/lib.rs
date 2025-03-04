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

//! Console informant. Prints sync progress and block events. Runs on the calling thread.

use anstyle::{AnsiColor, Reset};
use futures::prelude::*;
use futures_timer::Delay;
use log::{debug, info, trace};
use sc_client_api::{BlockchainEvents, UsageProvider};
use sc_network::NetworkStatusProvider;
use sc_network_common::sync::SyncStatusProvider;
use sp_blockchain::HeaderMetadata;
use sp_runtime::traits::{Block as BlockT, Header};
use std::{collections::VecDeque, fmt::Display, sync::Arc, time::Duration};

mod display;

/// Creates a stream that returns a new value every `duration`.
fn interval(duration: Duration) -> impl Stream<Item = ()> + Unpin {
	futures::stream::unfold((), move |_| Delay::new(duration).map(|_| Some(((), ())))).map(drop)
}

/// The format to print telemetry output in.
#[derive(Clone, Debug)]
pub struct OutputFormat {
	/// Enable color output in logs.
	///
	/// Is enabled by default.
	pub enable_color: bool,
}

impl Default for OutputFormat {
	fn default() -> Self {
		Self { enable_color: true }
	}
}

/// Builds the informant and returns a `Future` that drives the informant.
pub async fn build<B: BlockT, C, N, S>(client: Arc<C>, network: N, syncing: S, format: OutputFormat)
where
	N: NetworkStatusProvider,
	S: SyncStatusProvider<B>,
	C: UsageProvider<B> + HeaderMetadata<B> + BlockchainEvents<B>,
	<C as HeaderMetadata<B>>::Error: Display,
{
	let mut display = display::InformantDisplay::new(format.clone());

	let client_1 = client.clone();

	let display_notifications = interval(Duration::from_millis(5000))
		.filter_map(|_| async {
			let net_status = network.status().await;
			let sync_status = syncing.status().await;

			match (net_status.ok(), sync_status.ok()) {
				(Some(net), Some(sync)) => Some((net, sync)),
				_ => None,
			}
		})
		.for_each(move |(net_status, sync_status)| {
			let info = client_1.usage_info();
			if let Some(ref usage) = info.usage {
				trace!(target: "usage", "Usage statistics: {}", usage);
			} else {
				trace!(
					target: "usage",
					"Usage statistics not displayed as backend does not provide it",
				)
			}
			display.display(&info, net_status, sync_status);
			future::ready(())
		});

	futures::select! {
		() = display_notifications.fuse() => (),
		() = display_block_import(client).fuse() => (),
	};
}

fn display_block_import<B: BlockT, C>(client: Arc<C>) -> impl Future<Output = ()>
where
	C: UsageProvider<B> + HeaderMetadata<B> + BlockchainEvents<B>,
	<C as HeaderMetadata<B>>::Error: Display,
{
	let mut last_best = {
		let info = client.usage_info();
		Some((info.chain.best_number, info.chain.best_hash))
	};

	// Hashes of the last blocks we have seen at import.
	let mut last_blocks = VecDeque::new();
	let max_blocks_to_track = 100;

	client.import_notification_stream().for_each(move |n| {
		// detect and log reorganizations.
		if let Some((ref last_num, ref last_hash)) = last_best {
			if n.header.parent_hash() != last_hash && n.is_new_best {
				let maybe_ancestor =
					sp_blockchain::lowest_common_ancestor(&*client, *last_hash, n.hash);

				match maybe_ancestor {
					Ok(ref ancestor) if ancestor.hash != *last_hash => info!(
						"♻️  Reorg on #{},{} to #{},{}, common ancestor #{},{}",
						format!(
							"{}{}{}",
							AnsiColor::Red.on_default().bold().render(),
							last_num,
							Reset.render()
						),
						last_hash,
						format!(
							"{}{}{}",
							AnsiColor::Green.on_default().bold().render(),
							n.header.number(),
							Reset.render()
						),
						n.hash,
						format!(
							"{}{}{}",
							AnsiColor::White.on_default().bold().render(),
							ancestor.number,
							Reset.render()
						),
						ancestor.hash,
					),
					Ok(_) => {},
					Err(e) => debug!("Error computing tree route: {}", e),
				}
			}
		}

		if n.is_new_best {
			last_best = Some((*n.header.number(), n.hash));
		}

		// If we already printed a message for a given block recently,
		// we should not print it again.
		if !last_blocks.contains(&n.hash) {
			last_blocks.push_back(n.hash);

			if last_blocks.len() > max_blocks_to_track {
				last_blocks.pop_front();
			}

			info!(
				target: "substrate",
				"Imported #{} ({})",
				format!("{}{}{}", AnsiColor::White.on_default().bold().render(), n.header.number(), Reset.render()),
				n.hash,
			);
		}

		future::ready(())
	})
}
