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

#![cfg(feature = "runtime-benchmarks")]

use assert_cmd::cargo::cargo_bin;
use std::process::Command;

/// `benchmark pallet` works for the different combinations of `steps` and `repeat`.
#[test]
fn benchmark_pallet_works() {
	// Some invalid combinations:
	benchmark_pallet(0, 10, false);
	benchmark_pallet(1, 10, false);
	// ... and some valid:
	benchmark_pallet(2, 1, true);
	benchmark_pallet(50, 20, true);
	benchmark_pallet(20, 50, true);
}

fn benchmark_pallet(steps: u32, repeat: u32, should_work: bool) {
	let output = Command::new(cargo_bin("substrate"))
		.args(["benchmark", "pallet", "--dev"])
		// Use the `addition` benchmark since is the fastest.
		.args(["--pallet", "frame-benchmarking", "--extrinsic", "addition"])
		.args(["--steps", &format!("{}", steps), "--repeat", &format!("{}", repeat)])
		.output()
		.unwrap();

	if output.status.success() != should_work {
		let log = String::from_utf8_lossy(&output.stderr).to_string();
		panic!("Test failed:\n{}", log);
	}
}
