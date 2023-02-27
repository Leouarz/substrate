// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_elections_phragmen
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
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
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/substrate/.git/.artifacts/bench.json
// --pallet=pallet_elections_phragmen
// --chain=dev
// --header=./HEADER-APACHE2
// --output=./frame/elections-phragmen/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_elections_phragmen.
pub trait WeightInfo {
	fn vote_equal(v: u32, ) -> Weight;
	fn vote_more(v: u32, ) -> Weight;
	fn vote_less(v: u32, ) -> Weight;
	fn remove_voter() -> Weight;
	fn submit_candidacy(c: u32, ) -> Weight;
	fn renounce_candidacy_candidate(c: u32, ) -> Weight;
	fn renounce_candidacy_members() -> Weight;
	fn renounce_candidacy_runners_up() -> Weight;
	fn remove_member_without_replacement() -> Weight;
	fn remove_member_with_replacement() -> Weight;
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight;
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight;
}

/// Weights for pallet_elections_phragmen using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `499 + v * (80 ±0)`
		//  Estimated: `9726 + v * (320 ±0)`
		// Minimum execution time: 30_119 nanoseconds.
		Weight::from_ref_time(30_817_716)
			.saturating_add(Weight::from_proof_size(9726))
			// Standard Error: 2_178
			.saturating_add(Weight::from_ref_time(134_221).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_proof_size(320).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467 + v * (80 ±0)`
		//  Estimated: `9598 + v * (320 ±0)`
		// Minimum execution time: 40_800 nanoseconds.
		Weight::from_ref_time(41_560_970)
			.saturating_add(Weight::from_proof_size(9598))
			// Standard Error: 2_751
			.saturating_add(Weight::from_ref_time(109_976).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_proof_size(320).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `499 + v * (80 ±0)`
		//  Estimated: `9726 + v * (320 ±0)`
		// Minimum execution time: 40_628 nanoseconds.
		Weight::from_ref_time(41_471_540)
			.saturating_add(Weight::from_proof_size(9726))
			// Standard Error: 4_878
			.saturating_add(Weight::from_ref_time(160_594).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_proof_size(320).saturating_mul(v.into()))
	}
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `989`
		//  Estimated: `7238`
		// Minimum execution time: 37_028 nanoseconds.
		Weight::from_ref_time(37_379_000)
			.saturating_add(Weight::from_proof_size(7238))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 64]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1697 + c * (48 ±0)`
		//  Estimated: `6576 + c * (144 ±0)`
		// Minimum execution time: 32_794 nanoseconds.
		Weight::from_ref_time(33_648_162)
			.saturating_add(Weight::from_proof_size(6576))
			// Standard Error: 1_302
			.saturating_add(Weight::from_ref_time(73_514).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_proof_size(144).saturating_mul(c.into()))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 64]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `349 + c * (48 ±0)`
		//  Estimated: `844 + c * (48 ±0)`
		// Minimum execution time: 26_511 nanoseconds.
		Weight::from_ref_time(27_118_645)
			.saturating_add(Weight::from_proof_size(844))
			// Standard Error: 859
			.saturating_add(Weight::from_ref_time(49_424).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_proof_size(48).saturating_mul(c.into()))
	}
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2027`
		//  Estimated: `12115`
		// Minimum execution time: 44_881 nanoseconds.
		Weight::from_ref_time(45_414_000)
			.saturating_add(Weight::from_proof_size(12115))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `975`
		//  Estimated: `1470`
		// Minimum execution time: 28_818 nanoseconds.
		Weight::from_ref_time(29_209_000)
			.saturating_add(Weight::from_proof_size(1470))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000_000 nanoseconds.
		Weight::from_ref_time(2_000_000_000_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2027`
		//  Estimated: `14718`
		// Minimum execution time: 51_050 nanoseconds.
		Weight::from_ref_time(51_721_000)
			.saturating_add(Weight::from_proof_size(14718))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Elections Voting (r:513 w:512)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:512 w:512)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:512 w:512)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `v` is `[256, 512]`.
	/// The range of component `d` is `[0, 256]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1115 + v * (875 ±0)`
		//  Estimated: `8448 + v * (12352 ±0)`
		// Minimum execution time: 16_508_164 nanoseconds.
		Weight::from_ref_time(16_572_267_000)
			.saturating_add(Weight::from_proof_size(8448))
			// Standard Error: 275_995
			.saturating_add(Weight::from_ref_time(38_979_205).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_proof_size(12352).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:513 w:0)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:44 w:44)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Elections ElectionRounds (r:1 w:1)
	/// Proof Skipped: Elections ElectionRounds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 64]`.
	/// The range of component `v` is `[1, 512]`.
	/// The range of component `e` is `[512, 8192]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + v * (638 ±0) + e * (28 ±0)`
		//  Estimated: `330033 + v * (5229 ±6) + e * (89 ±0) + c * (2135 ±7)`
		// Minimum execution time: 1_243_131 nanoseconds.
		Weight::from_ref_time(1_252_274_000)
			.saturating_add(Weight::from_proof_size(330033))
			// Standard Error: 599_529
			.saturating_add(Weight::from_ref_time(19_976_106).saturating_mul(v.into()))
			// Standard Error: 38_467
			.saturating_add(Weight::from_ref_time(810_279).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(21_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_proof_size(5229).saturating_mul(v.into()))
			.saturating_add(Weight::from_proof_size(89).saturating_mul(e.into()))
			.saturating_add(Weight::from_proof_size(2135).saturating_mul(c.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `499 + v * (80 ±0)`
		//  Estimated: `9726 + v * (320 ±0)`
		// Minimum execution time: 30_119 nanoseconds.
		Weight::from_ref_time(30_817_716)
			.saturating_add(Weight::from_proof_size(9726))
			// Standard Error: 2_178
			.saturating_add(Weight::from_ref_time(134_221).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_proof_size(320).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467 + v * (80 ±0)`
		//  Estimated: `9598 + v * (320 ±0)`
		// Minimum execution time: 40_800 nanoseconds.
		Weight::from_ref_time(41_560_970)
			.saturating_add(Weight::from_proof_size(9598))
			// Standard Error: 2_751
			.saturating_add(Weight::from_ref_time(109_976).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_proof_size(320).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `499 + v * (80 ±0)`
		//  Estimated: `9726 + v * (320 ±0)`
		// Minimum execution time: 40_628 nanoseconds.
		Weight::from_ref_time(41_471_540)
			.saturating_add(Weight::from_proof_size(9726))
			// Standard Error: 4_878
			.saturating_add(Weight::from_ref_time(160_594).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_proof_size(320).saturating_mul(v.into()))
	}
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `989`
		//  Estimated: `7238`
		// Minimum execution time: 37_028 nanoseconds.
		Weight::from_ref_time(37_379_000)
			.saturating_add(Weight::from_proof_size(7238))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 64]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1697 + c * (48 ±0)`
		//  Estimated: `6576 + c * (144 ±0)`
		// Minimum execution time: 32_794 nanoseconds.
		Weight::from_ref_time(33_648_162)
			.saturating_add(Weight::from_proof_size(6576))
			// Standard Error: 1_302
			.saturating_add(Weight::from_ref_time(73_514).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_proof_size(144).saturating_mul(c.into()))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 64]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `349 + c * (48 ±0)`
		//  Estimated: `844 + c * (48 ±0)`
		// Minimum execution time: 26_511 nanoseconds.
		Weight::from_ref_time(27_118_645)
			.saturating_add(Weight::from_proof_size(844))
			// Standard Error: 859
			.saturating_add(Weight::from_ref_time(49_424).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_proof_size(48).saturating_mul(c.into()))
	}
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2027`
		//  Estimated: `12115`
		// Minimum execution time: 44_881 nanoseconds.
		Weight::from_ref_time(45_414_000)
			.saturating_add(Weight::from_proof_size(12115))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `975`
		//  Estimated: `1470`
		// Minimum execution time: 28_818 nanoseconds.
		Weight::from_ref_time(29_209_000)
			.saturating_add(Weight::from_proof_size(1470))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000_000 nanoseconds.
		Weight::from_ref_time(2_000_000_000_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2027`
		//  Estimated: `14718`
		// Minimum execution time: 51_050 nanoseconds.
		Weight::from_ref_time(51_721_000)
			.saturating_add(Weight::from_proof_size(14718))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: Elections Voting (r:513 w:512)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:512 w:512)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:512 w:512)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `v` is `[256, 512]`.
	/// The range of component `d` is `[0, 256]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1115 + v * (875 ±0)`
		//  Estimated: `8448 + v * (12352 ±0)`
		// Minimum execution time: 16_508_164 nanoseconds.
		Weight::from_ref_time(16_572_267_000)
			.saturating_add(Weight::from_proof_size(8448))
			// Standard Error: 275_995
			.saturating_add(Weight::from_ref_time(38_979_205).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(v.into())))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_proof_size(12352).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:513 w:0)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:44 w:44)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Elections ElectionRounds (r:1 w:1)
	/// Proof Skipped: Elections ElectionRounds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 64]`.
	/// The range of component `v` is `[1, 512]`.
	/// The range of component `e` is `[512, 8192]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + v * (638 ±0) + e * (28 ±0)`
		//  Estimated: `330033 + v * (5229 ±6) + e * (89 ±0) + c * (2135 ±7)`
		// Minimum execution time: 1_243_131 nanoseconds.
		Weight::from_ref_time(1_252_274_000)
			.saturating_add(Weight::from_proof_size(330033))
			// Standard Error: 599_529
			.saturating_add(Weight::from_ref_time(19_976_106).saturating_mul(v.into()))
			// Standard Error: 38_467
			.saturating_add(Weight::from_ref_time(810_279).saturating_mul(e.into()))
			.saturating_add(RocksDbWeight::get().reads(21_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_proof_size(5229).saturating_mul(v.into()))
			.saturating_add(Weight::from_proof_size(89).saturating_mul(e.into()))
			.saturating_add(Weight::from_proof_size(2135).saturating_mul(c.into()))
	}
}
