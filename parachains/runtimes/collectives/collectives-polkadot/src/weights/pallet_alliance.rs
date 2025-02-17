// Copyright Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_alliance`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_alliance
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_alliance.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_alliance`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_alliance::WeightInfo for WeightInfo<T> {
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalCount (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Voting (r:0 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `439 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `14703 + m * (128 ±0) + p * (144 ±0)`
		// Minimum execution time: 29_793_000 picoseconds.
		Weight::from_parts(31_622_009, 0)
			.saturating_add(Weight::from_parts(0, 14703))
			// Standard Error: 59
			.saturating_add(Weight::from_parts(194, 0).saturating_mul(b.into()))
			// Standard Error: 626
			.saturating_add(Weight::from_parts(16_277, 0).saturating_mul(m.into()))
			// Standard Error: 618
			.saturating_add(Weight::from_parts(115_342, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 128).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 144).saturating_mul(p.into()))
	}
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `868 + m * (64 ±0)`
		//  Estimated: `11008 + m * (64 ±0)`
		// Minimum execution time: 24_860_000 picoseconds.
		Weight::from_parts(25_540_583, 0)
			.saturating_add(Weight::from_parts(0, 11008))
			// Standard Error: 1_049
			.saturating_add(Weight::from_parts(44_450, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312 + m * (96 ±0) + p * (36 ±0)`
		//  Estimated: `14519 + m * (388 ±0) + p * (144 ±0)`
		// Minimum execution time: 35_120_000 picoseconds.
		Weight::from_parts(33_274_472, 0)
			.saturating_add(Weight::from_parts(0, 14519))
			// Standard Error: 1_030
			.saturating_add(Weight::from_parts(41_853, 0).saturating_mul(m.into()))
			// Standard Error: 1_004
			.saturating_add(Weight::from_parts(111_626, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 388).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 144).saturating_mul(p.into()))
	}
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(_b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `762 + m * (96 ±0) + p * (41 ±0)`
		//  Estimated: `19732 + m * (388 ±0) + p * (160 ±0)`
		// Minimum execution time: 47_135_000 picoseconds.
		Weight::from_parts(42_191_348, 0)
			.saturating_add(Weight::from_parts(0, 19732))
			// Standard Error: 4_449
			.saturating_add(Weight::from_parts(76_548, 0).saturating_mul(m.into()))
			// Standard Error: 4_337
			.saturating_add(Weight::from_parts(143_406, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 388).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 160).saturating_mul(p.into()))
	}
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:1 w:0)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Alliance Rule (r:0 w:1)
	/// Proof: Alliance Rule (max_values: Some(1), max_size: Some(87), added: 582, mode: MaxEncodedLen)
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `518 + m * (96 ±0) + p * (41 ±0)`
		//  Estimated: `19135 + m * (509 ±0) + p * (203 ±0)`
		// Minimum execution time: 45_329_000 picoseconds.
		Weight::from_parts(46_859_108, 0)
			.saturating_add(Weight::from_parts(0, 19135))
			// Standard Error: 3_831
			.saturating_add(Weight::from_parts(100_587, 0).saturating_mul(m.into()))
			// Standard Error: 3_785
			.saturating_add(Weight::from_parts(134_469, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 509).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 203).saturating_mul(p.into()))
	}
	/// Storage: Alliance Members (r:1 w:0)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:1 w:0)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(_b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `417 + m * (96 ±0) + p * (36 ±0)`
		//  Estimated: `16746 + m * (480 ±0) + p * (180 ±0)`
		// Minimum execution time: 36_795_000 picoseconds.
		Weight::from_parts(35_568_715, 0)
			.saturating_add(Weight::from_parts(0, 16746))
			// Standard Error: 630
			.saturating_add(Weight::from_parts(42_253, 0).saturating_mul(m.into()))
			// Standard Error: 607
			.saturating_add(Weight::from_parts(107_080, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 480).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 180).saturating_mul(p.into()))
	}
	/// Storage: Alliance Members (r:2 w:2)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Members (r:1 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `z` is `[0, 100]`.
	fn init_members(m: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12`
		//  Estimated: `13859`
		// Minimum execution time: 30_057_000 picoseconds.
		Weight::from_parts(19_584_887, 0)
			.saturating_add(Weight::from_parts(0, 13859))
			// Standard Error: 657
			.saturating_add(Weight::from_parts(127_220, 0).saturating_mul(m.into()))
			// Standard Error: 649
			.saturating_add(Weight::from_parts(111_719, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Alliance Members (r:2 w:2)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Alliance DepositOf (r:200 w:50)
	/// Proof: Alliance DepositOf (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: System Account (r:50 w:50)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Members (r:0 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 100]`.
	/// The range of component `y` is `[0, 100]`.
	/// The range of component `z` is `[0, 50]`.
	fn disband(x: u32, y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + x * (52 ±0) + y * (53 ±0) + z * (250 ±0)`
		//  Estimated: `35354 + x * (2590 ±0) + y * (2590 ±0) + z * (3104 ±1)`
		// Minimum execution time: 257_465_000 picoseconds.
		Weight::from_parts(258_993_000, 0)
			.saturating_add(Weight::from_parts(0, 35354))
			// Standard Error: 22_094
			.saturating_add(Weight::from_parts(469_539, 0).saturating_mul(x.into()))
			// Standard Error: 21_987
			.saturating_add(Weight::from_parts(478_507, 0).saturating_mul(y.into()))
			// Standard Error: 43_935
			.saturating_add(Weight::from_parts(10_541_803, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(y.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(z.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(z.into())))
			.saturating_add(Weight::from_parts(0, 2590).saturating_mul(x.into()))
			.saturating_add(Weight::from_parts(0, 2590).saturating_mul(y.into()))
			.saturating_add(Weight::from_parts(0, 3104).saturating_mul(z.into()))
	}
	/// Storage: Alliance Rule (r:0 w:1)
	/// Proof: Alliance Rule (max_values: Some(1), max_size: Some(87), added: 582, mode: MaxEncodedLen)
	fn set_rule() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_973_000 picoseconds.
		Weight::from_parts(10_247_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Alliance Announcements (r:1 w:1)
	/// Proof: Alliance Announcements (max_values: Some(1), max_size: Some(8702), added: 9197, mode: MaxEncodedLen)
	fn announce() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `10187`
		// Minimum execution time: 12_510_000 picoseconds.
		Weight::from_parts(12_659_000, 0)
			.saturating_add(Weight::from_parts(0, 10187))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Alliance Announcements (r:1 w:1)
	/// Proof: Alliance Announcements (max_values: Some(1), max_size: Some(8702), added: 9197, mode: MaxEncodedLen)
	fn remove_announcement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `149`
		//  Estimated: `10187`
		// Minimum execution time: 13_365_000 picoseconds.
		Weight::from_parts(13_575_000, 0)
			.saturating_add(Weight::from_parts(0, 10187))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Alliance Members (r:3 w:1)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	/// Proof: Alliance UnscrupulousAccounts (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Alliance DepositOf (r:0 w:1)
	/// Proof: Alliance DepositOf (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	fn join_alliance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `294`
		//  Estimated: `26328`
		// Minimum execution time: 40_044_000 picoseconds.
		Weight::from_parts(41_623_000, 0)
			.saturating_add(Weight::from_parts(0, 26328))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Alliance Members (r:3 w:1)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	/// Proof: Alliance UnscrupulousAccounts (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	fn nominate_ally() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `193`
		//  Estimated: `22735`
		// Minimum execution time: 28_166_000 picoseconds.
		Weight::from_parts(28_756_000, 0)
			.saturating_add(Weight::from_parts(0, 22735))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Alliance Members (r:2 w:2)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:0 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	fn elevate_ally() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236`
		//  Estimated: `14555`
		// Minimum execution time: 25_759_000 picoseconds.
		Weight::from_parts(26_083_000, 0)
			.saturating_add(Weight::from_parts(0, 14555))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Alliance Members (r:4 w:2)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:0 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Alliance RetiringMembers (r:0 w:1)
	/// Proof: Alliance RetiringMembers (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn give_retirement_notice() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236`
		//  Estimated: `25927`
		// Minimum execution time: 32_603_000 picoseconds.
		Weight::from_parts(33_091_000, 0)
			.saturating_add(Weight::from_parts(0, 25927))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Alliance RetiringMembers (r:1 w:1)
	/// Proof: Alliance RetiringMembers (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Alliance Members (r:1 w:1)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: Alliance DepositOf (r:1 w:1)
	/// Proof: Alliance DepositOf (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn retire() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `517`
		//  Estimated: `17315`
		// Minimum execution time: 36_169_000 picoseconds.
		Weight::from_parts(36_746_000, 0)
			.saturating_add(Weight::from_parts(0, 17315))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Alliance Members (r:3 w:1)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Alliance DepositOf (r:1 w:1)
	/// Proof: Alliance DepositOf (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:0 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	fn kick_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `622`
		//  Estimated: `45128`
		// Minimum execution time: 127_845_000 picoseconds.
		Weight::from_parts(129_248_000, 0)
			.saturating_add(Weight::from_parts(0, 45128))
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	/// Proof: Alliance UnscrupulousAccounts (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// Proof: Alliance UnscrupulousWebsites (max_values: Some(1), max_size: Some(25702), added: 26197, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `31874`
		// Minimum execution time: 8_183_000 picoseconds.
		Weight::from_parts(8_256_000, 0)
			.saturating_add(Weight::from_parts(0, 31874))
			// Standard Error: 2_929
			.saturating_add(Weight::from_parts(1_444_558, 0).saturating_mul(n.into()))
			// Standard Error: 1_147
			.saturating_add(Weight::from_parts(68_146, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	/// Proof: Alliance UnscrupulousAccounts (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// Proof: Alliance UnscrupulousWebsites (max_values: Some(1), max_size: Some(25702), added: 26197, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + n * (289 ±0) + l * (100 ±0)`
		//  Estimated: `31874`
		// Minimum execution time: 7_982_000 picoseconds.
		Weight::from_parts(8_084_000, 0)
			.saturating_add(Weight::from_parts(0, 31874))
			// Standard Error: 185_716
			.saturating_add(Weight::from_parts(16_937_748, 0).saturating_mul(n.into()))
			// Standard Error: 72_734
			.saturating_add(Weight::from_parts(291_993, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Alliance Members (r:3 w:2)
	/// Proof: Alliance Members (max_values: None, max_size: Some(3211), added: 5686, mode: MaxEncodedLen)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:0 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	fn abdicate_fellow_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236`
		//  Estimated: `20241`
		// Minimum execution time: 31_916_000 picoseconds.
		Weight::from_parts(32_301_000, 0)
			.saturating_add(Weight::from_parts(0, 20241))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
