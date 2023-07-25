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

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::{CoreAssignment::Task, Pallet as Broker};
use frame_benchmarking::v2::*;
use frame_support::{
	storage::bounded_vec::BoundedVec,
	traits::{fungible::Mutate, EnsureOrigin, Hooks},
};
use frame_system::{Pallet as System, RawOrigin};
use sp_arithmetic::Perbill;
use sp_core::Get;
use sp_runtime::Saturating;

const SEED: u32 = 0;
const MAX_CORE_COUNT: u16 = 1_000;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn new_config_record<T: Config>() -> ConfigRecordOf<T> {
	ConfigRecord {
		advance_notice: 2u32.into(),
		interlude_length: 1u32.into(),
		leadin_length: 1u32.into(),
		ideal_bulk_proportion: Default::default(),
		limit_cores_offered: None,
		region_length: 3,
		renewal_bump: Perbill::from_percent(10),
		contribution_timeout: 5,
	}
}

fn new_schedule() -> Schedule {
	// Max items for worst case
	let mut items = Vec::new();
	for i in 0..CORE_MASK_BITS {
		items.push(ScheduleItem {
			assignment: Task(i.try_into().unwrap()),
			part: CoreMask::complete(),
		});
	}
	Schedule::truncate_from(items)
}

fn setup_reservations<T: Config>(n: u32) {
	let schedule = new_schedule();

	Reservations::<T>::put(BoundedVec::try_from(vec![schedule.clone(); n as usize]).unwrap());
}

fn setup_leases<T: Config>(n: u32, task: u32, until: u32) {
	Leases::<T>::put(
		BoundedVec::try_from(vec![LeaseRecordItem { task, until: until.into() }; n as usize])
			.unwrap(),
	);
}

fn advance_to<T: Config>(b: u32) {
	while System::<T>::block_number() < b.into() {
		System::<T>::set_block_number(System::<T>::block_number().saturating_add(1u32.into()));
		Broker::<T>::on_initialize(System::<T>::block_number());
	}
}

fn setup_and_start_sale<T: Config>() -> Result<u16, BenchmarkError> {
	Configuration::<T>::put(new_config_record::<T>());

	// Assume Reservations to be filled for worst case
	setup_reservations::<T>(T::MaxReservedCores::get());

	// Assume Leases to be filled for worst case
	setup_leases::<T>(T::MaxLeasedCores::get(), 1, 10);

	Broker::<T>::do_start_sales(10u32.into(), MAX_CORE_COUNT.into())
		.map_err(|_| BenchmarkError::Weightless)?;

	Ok(T::MaxReservedCores::get()
		.saturating_add(T::MaxLeasedCores::get())
		.try_into()
		.unwrap())
}

#[benchmarks]
mod benches {
	use super::*;
	use crate::Finality::Final;

	#[benchmark]
	fn configure() -> Result<(), BenchmarkError> {
		let config = new_config_record::<T>();

		let origin =
			T::AdminOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(origin as T::RuntimeOrigin, config.clone());

		assert_eq!(Configuration::<T>::get(), Some(config));

		Ok(())
	}

	#[benchmark]
	fn reserve() -> Result<(), BenchmarkError> {
		let schedule = new_schedule();

		// Assume Reservations to be almost filled for worst case
		setup_reservations::<T>(T::MaxReservedCores::get().saturating_sub(1));

		let origin =
			T::AdminOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(origin as T::RuntimeOrigin, schedule);

		assert_eq!(Reservations::<T>::get().len(), T::MaxReservedCores::get() as usize);

		Ok(())
	}

	#[benchmark]
	fn unreserve(
		n: Linear<0, { T::MaxReservedCores::get().saturating_sub(1) }>,
	) -> Result<(), BenchmarkError> {
		// Assume Reservations to be filled for worst case
		setup_reservations::<T>(T::MaxReservedCores::get());

		let origin =
			T::AdminOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(origin as T::RuntimeOrigin, n);

		assert_eq!(
			Reservations::<T>::get().len(),
			T::MaxReservedCores::get().saturating_sub(1) as usize
		);

		Ok(())
	}

	#[benchmark]
	fn set_lease() -> Result<(), BenchmarkError> {
		let task = 1u32;
		let until = 10u32.into();

		// Assume Leases to be almost filled for worst case
		setup_leases::<T>(T::MaxLeasedCores::get().saturating_sub(1), task, until);

		let origin =
			T::AdminOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(origin as T::RuntimeOrigin, task, until);

		assert_eq!(Leases::<T>::get().len(), T::MaxLeasedCores::get() as usize);

		Ok(())
	}

	#[benchmark]
	fn start_sales(n: Linear<0, { MAX_CORE_COUNT.into() }>) -> Result<(), BenchmarkError> {
		Configuration::<T>::put(new_config_record::<T>());

		// Assume Reservations to be filled for worst case
		setup_reservations::<T>(T::MaxReservedCores::get());

		// Assume Leases to be filled for worst case
		setup_leases::<T>(T::MaxLeasedCores::get(), 1, 10);

		let initial_price = 10u32.into();

		let origin =
			T::AdminOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(origin as T::RuntimeOrigin, initial_price, n.try_into().unwrap());

		assert!(SaleInfo::<T>::get().is_some());
		assert_last_event::<T>(
			Event::SaleInitialized {
				sale_start: 2u32.into(),
				leadin_length: 1u32.into(),
				start_price: 20u32.into(),
				regular_price: 10u32.into(),
				region_begin: 4,
				region_end: 7,
				ideal_cores_sold: 0,
				cores_offered: n
					.saturating_sub(T::MaxReservedCores::get())
					.saturating_sub(T::MaxLeasedCores::get())
					.try_into()
					.unwrap(),
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn purchase() -> Result<(), BenchmarkError> {
		let core = setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 10u32.into());

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), 10u32.into());

		assert_eq!(SaleInfo::<T>::get().unwrap().sellout_price, Some(10u32.into()));
		assert_last_event::<T>(
			Event::Purchased {
				who: caller,
				region_id: RegionId { begin: 4, core, part: CoreMask::complete() },
				price: 10u32.into(),
				duration: 3u32.into(),
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn renew() -> Result<(), BenchmarkError> {
		setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 20u32.into());

		let region = Broker::<T>::do_purchase(caller.clone(), 10u32.into())
			.map_err(|_| BenchmarkError::Weightless)?;

		Broker::<T>::do_assign(region, None, 1001, Final)
			.map_err(|_| BenchmarkError::Weightless)?;

		advance_to::<T>(6);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), region.core);

		let id = AllowedRenewalId { core: region.core, when: 10 };
		assert!(AllowedRenewals::<T>::get(id).is_some());

		Ok(())
	}

	#[benchmark]
	fn transfer() -> Result<(), BenchmarkError> {
		setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 10u32.into());

		let region = Broker::<T>::do_purchase(caller.clone(), 10u32.into())
			.map_err(|_| BenchmarkError::Weightless)?;

		let recipient: T::AccountId = account("recipient", 0, SEED);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), region, recipient);

		Ok(())
	}

	#[benchmark]
	fn partition() -> Result<(), BenchmarkError> {
		setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 10u32.into());

		let region = Broker::<T>::do_purchase(caller.clone(), 10u32.into())
			.map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), region, 2);

		Ok(())
	}

	#[benchmark]
	fn interlace() -> Result<(), BenchmarkError> {
		let core = setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 10u32.into());

		let region = Broker::<T>::do_purchase(caller.clone(), 10u32.into())
			.map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), region, 0x00000_fffff_fffff_00000.into());

		assert_last_event::<T>(
			Event::Interlaced {
				old_region_id: RegionId { begin: 4, core, part: CoreMask::complete() },
				new_region_ids: (
					RegionId { begin: 4, core, part: 0x00000_fffff_fffff_00000.into() },
					RegionId {
						begin: 4,
						core,
						part: CoreMask::complete() ^ 0x00000_fffff_fffff_00000.into(),
					},
				),
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn assign() -> Result<(), BenchmarkError> {
		let core = setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 10u32.into());

		let region = Broker::<T>::do_purchase(caller.clone(), 10u32.into())
			.map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), region, 1000, Final);

		let workplan_key = (region.begin, region.core);
		assert!(Workplan::<T>::get(workplan_key).is_some());

		assert_last_event::<T>(
			Event::Assigned {
				region_id: RegionId { begin: 4, core, part: CoreMask::complete() },
				task: 1000,
				duration: 3u32.into(),
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn pool() -> Result<(), BenchmarkError> {
		let core = setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 10u32.into());

		let region = Broker::<T>::do_purchase(caller.clone(), 10u32.into())
			.map_err(|_| BenchmarkError::Weightless)?;

		let recipient: T::AccountId = account("recipient", 0, SEED);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), region, recipient, Final);

		let workplan_key = (region.begin, region.core);
		assert!(Workplan::<T>::get(workplan_key).is_some());

		assert_last_event::<T>(
			Event::Pooled {
				region_id: RegionId { begin: 4, core, part: CoreMask::complete() },
				duration: 3u32.into(),
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn claim_revenue() -> Result<(), BenchmarkError> {
		setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 10u32.into());

		let region = Broker::<T>::do_purchase(caller.clone(), 10u32.into())
			.map_err(|_| BenchmarkError::Weightless)?;

		let recipient: T::AccountId = account("recipient", 0, SEED);

		Broker::<T>::do_pool(region, None, recipient, Final)
			.map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), region, 100);

		Ok(())
	}

	#[benchmark]
	fn purchase_credit() -> Result<(), BenchmarkError> {
		setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 30u32.into());

		let region = Broker::<T>::do_purchase(caller.clone(), 10u32.into())
			.map_err(|_| BenchmarkError::Weightless)?;

		let recipient: T::AccountId = account("recipient", 0, SEED);

		Broker::<T>::do_pool(region, None, recipient, Final)
			.map_err(|_| BenchmarkError::Weightless)?;

		let beneficiary: RelayAccountIdOf<T> = account("beneficiary", 0, SEED);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), 20u32.into(), beneficiary.clone());

		assert_last_event::<T>(
			Event::CreditPurchased { who: caller, beneficiary, amount: 20u32.into() }.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn drop_region() -> Result<(), BenchmarkError> {
		let core = setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 10u32.into());

		let region = Broker::<T>::do_purchase(caller.clone(), 10u32.into())
			.map_err(|_| BenchmarkError::Weightless)?;

		advance_to::<T>(12);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), region);

		assert_last_event::<T>(
			Event::RegionDropped {
				region_id: RegionId { begin: 4, core, part: CoreMask::complete() },
				duration: 3u32.into(),
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn drop_contribution() -> Result<(), BenchmarkError> {
		let core = setup_and_start_sale::<T>()?;

		advance_to::<T>(2);

		let caller: T::AccountId = whitelisted_caller();
		T::Currency::set_balance(&caller.clone(), 10u32.into());

		let region = Broker::<T>::do_purchase(caller.clone(), 10u32.into())
			.map_err(|_| BenchmarkError::Weightless)?;

		let recipient: T::AccountId = account("recipient", 0, SEED);

		Broker::<T>::do_pool(region, None, recipient, Final)
			.map_err(|_| BenchmarkError::Weightless)?;

		advance_to::<T>(26);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), region);

		assert_last_event::<T>(
			Event::ContributionDropped {
				region_id: RegionId { begin: 4, core, part: CoreMask::complete() },
			}
			.into(),
		);

		Ok(())
	}

	#[benchmark]
	fn request_core_count(n: Linear<0, { MAX_CORE_COUNT.into() }>) -> Result<(), BenchmarkError> {
		let admin_origin =
			T::AdminOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(admin_origin as T::RuntimeOrigin, n.try_into().unwrap());

		assert_last_event::<T>(
			Event::CoreCountRequested { core_count: n.try_into().unwrap() }.into(),
		);

		Ok(())
	}

	// Implements a test for each benchmark. Execute with:
	// `cargo test -p pallet-broker --features runtime-benchmarks`.
	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}