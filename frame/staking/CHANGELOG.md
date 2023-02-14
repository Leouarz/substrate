# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The semantic versioning guarantees cover the interface to the substrate runtime which
includes this pallet as a dependency. This module will also add storage migrations whenever
changes require it. Stability with regard to offchain tooling is explicitly excluded from
this guarantee: For example adding a new field to an in-storage data structure will require
changes to frontends to properly display it. However, those changes will still be regarded
as a minor version bump.

## [5.0.0] - UNRELEASED

### Added

- Unlimited number of nominators can be rewarded.
- New storage item `ExposurePageSize` to limit the number of nominators rewarded for a single call for reward payout.
- New storage item `MaxExposurePageCount` to limit the maximum number of exposure pages that can be created. If this
- value is not set, it defaults to 1, resulting in the same behaviour as we have today with clipped exposures.
- New storage item `ErasStakersPaged` that keeps upto `ExposurePageSize` individual nominator exposures by era, validator and page.
- New storage item `ErasStakersOverview` which complements `ErasStakersPaged` and keeps track of validator's own stake and total backing stake for each era.
- New call `payout_stakers_by_page` that allows to payout rewards for a single validator by passing the page explicitly.
- New storage item `ClaimedRewards` that keeps track of claimed reward history of a validator by era and page.

### Changed
- `payout_stakers` can be called multiple times for the same era if the validator has more than `ExposurePageSize` nominators backing them.
- `MaxNominatorRewardedPerValidator` is renamed to `MaxExposurePageSize`.

### Deprecated
- `ErasStakersClipped` is deprecated in favor of `ErasStakersPaged`. In 84 eras, `ErasStakersClipped` will be removed.
- `StakingLedger.claimed_rewards` is renamed to `StakingLedger.legacy_claimed_rewards` and is deprecated.


[5.0.0]: https://github.com/paritytech/substrate/pull/13059