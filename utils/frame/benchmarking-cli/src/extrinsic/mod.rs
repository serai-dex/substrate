// This file is part of a fork of Substrate which has had various changes.

// Copyright (C) Parity Technologies (UK) Ltd.
// Copyright (C) 2022-2023 Luke Parker
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

//! Benchmark the time it takes to execute a specific extrinsic.
//! This is a generalization of the *overhead* benchmark which can only measure `System::Remark`
//! extrinsics.

pub mod bench;
pub mod cmd;
pub mod extrinsic_factory;

pub use cmd::ExtrinsicCmd;
pub use extrinsic_factory::{ExtrinsicBuilder, ExtrinsicFactory};
