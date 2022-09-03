use core::{marker::PhantomData, result};

use codec::{Decode, Encode, EncodeLike, FullCodec};
use frame_metadata::{StorageEntryMetadata, StorageEntryType};
use scale_info::TypeInfo;
use sp_arithmetic::traits::Bounded;

use crate::{
	storage::{self, StorageAppend, StorageDecodeLength, StorageTryAppend},
	traits::{Get, OnUnbalanced, StorageInfo, StorageInstance},
	StoragePrefixedMap,
};
// we don't bring this fully into scope because it can be confusing -- only to allow trait functions
// being used.
use storage::generator::StorageMap as _;

use super::{QueryKindTrait, StorageEntryMetadataBuilder};

// / This is fired IFF some value already existed in `key`.
// #[impl_trait_for_tuples::impl_for_tuples(0, 32)]
pub trait StorageOnRemove<K: FullCodec, V> {
	fn on_remove<KeyArg: EncodeLike<K>>(key: &KeyArg, value: &V);
}

// #[impl_trait_for_tuples::impl_for_tuples(0, 32)]
pub trait StorageOnInsert<K: FullCodec, V> {
	fn on_insert<KeyArg: EncodeLike<K>>(key: &KeyArg, value: &V);
}

// #[impl_trait_for_tuples::impl_for_tuples(0, 32)]
pub trait StorageOnUpdate<K: FullCodec, V> {
	fn on_update<KeyArg: EncodeLike<K>>(key: &KeyArg, old_value: &V, new_value: &V);
}

pub struct HookedMap<Map, Key, Value, OnRemove = (), OnInsert = (), OnUpdate = ()>(
	PhantomData<(Map, Key, Value, OnRemove, OnInsert, OnUpdate)>,
);

impl<Key, Value, Map, OnRemove, OnInsert, OnUpdate> storage::generator::StorageMap<Key, Value>
	for HookedMap<Map, Key, Value, OnRemove, OnInsert, OnUpdate>
where
	Key: FullCodec,
	Value: FullCodec,
	Map: storage::generator::StorageMap<Key, Value>,
{
	type Query = <Map as storage::StorageMap<Key, Value>>::Query;
	type Hasher = Map::Hasher;
	fn module_prefix() -> &'static [u8] {
		Map::module_prefix()
	}
	fn storage_prefix() -> &'static [u8] {
		Map::storage_prefix()
	}
	fn from_optional_value_to_query(v: Option<Value>) -> Self::Query {
		Map::from_optional_value_to_query(v)
	}
	fn from_query_to_optional_value(v: Self::Query) -> Option<Value> {
		Map::from_query_to_optional_value(v)
	}
}

impl<Key, Value, Map, OnRemove, OnInsert, OnUpdate> StoragePrefixedMap<Value>
	for HookedMap<Map, Key, Value, OnRemove, OnInsert, OnUpdate>
where
	Value: FullCodec,
	Map: StoragePrefixedMap<Value>,
{
	fn module_prefix() -> &'static [u8] {
		Map::module_prefix()
	}
	fn storage_prefix() -> &'static [u8] {
		Map::storage_prefix()
	}
}

impl<Key, Value, Map, OnRemove, OnInsert, OnUpdate>
	HookedMap<Map, Key, Value, OnRemove, OnInsert, OnUpdate>
where
	OnRemove: StorageOnRemove<Key, Value>,
	OnInsert: StorageOnInsert<Key, Value>,
	OnUpdate: StorageOnUpdate<Key, Value>,
	Key: FullCodec,
	Value: FullCodec + Clone,
	Map: storage::StorageMap<Key, Value>
		+ storage::generator::StorageMap<Key, Value>
		+ StoragePrefixedMap<Value>,
	<Map as storage::StorageMap<Key, Value>>::Query: Clone,
{
	/// Maybe get the value for the given key from the map.
	///
	/// Returns `Some` if it exists, `None` if not.
	///
	/// This is not publicly available, since it is equivalent to `get`.
	fn maybe_get<KeyArg: EncodeLike<Key>>(key: KeyArg) -> Option<Value> {
		Self::try_get(key).ok()
	}

	fn post_mutate_hooks<KeyArg: EncodeLike<Key>>(
		key: KeyArg,
		maybe_old_value: Option<Value>,
		maybe_new_value: Option<Value>,
	) {
		match (maybe_old_value, maybe_new_value) {
			(Some(old_value), Some(new_value)) => {
				OnUpdate::on_update(&key, &old_value, &new_value);
			},
			(Some(old_value), None) => {
				OnRemove::on_remove(&key, &old_value);
			},
			(None, Some(new_value)) => {
				OnInsert::on_insert(&key, &new_value);
			},
			(None, None) => {},
		}
	}

	/// Get the storage key used to fetch a value corresponding to a specific key.
	pub fn hashed_key_for<KeyArg: EncodeLike<Key>>(key: KeyArg) -> Vec<u8> {
		<Map as storage::StorageMap<Key, Value>>::hashed_key_for(key)
	}

	/// Does the value (explicitly) exist in storage?
	pub fn contains_key<KeyArg: EncodeLike<Key>>(key: KeyArg) -> bool {
		<Map as storage::StorageMap<Key, Value>>::contains_key(key)
	}

	/// Load the value associated with the given key from the map.
	pub fn get<KeyArg: EncodeLike<Key>>(
		key: KeyArg,
	) -> <Map as storage::StorageMap<Key, Value>>::Query {
		<Map as storage::StorageMap<Key, Value>>::get(key)
	}

	/// Try to get the value for the given key from the map.
	///
	/// Returns `Ok` if it exists, `Err` if not.
	pub fn try_get<KeyArg: EncodeLike<Key>>(key: KeyArg) -> Result<Value, ()> {
		<Map as storage::StorageMap<Key, Value>>::try_get(key)
	}

	/// Swap the values of two keys.
	pub fn swap<KeyArg1: EncodeLike<Key> + Clone, KeyArg2: EncodeLike<Key> + Clone>(
		key1: KeyArg1,
		key2: KeyArg2,
	) {
		let maybe_value1 = Self::maybe_get(key1.clone());
		let maybe_value2 = Self::maybe_get(key2.clone());
		match (maybe_value1, maybe_value2) {
			(Some(value1), Some(value2)) => {
				// Both existed, and now swapped.
				OnUpdate::on_update(&key1, &value1, &value2);
				OnUpdate::on_update(&key2, &value2, &value1);
			},
			(Some(value1), None) => {
				// val1 will be removed, val2 will be created.
				OnRemove::on_remove(&key1, &value1);
				OnInsert::on_insert(&key2, &value1);
			},
			(None, Some(value2)) => {
				// val2 will be removed, val1 will be created.
				OnRemove::on_remove(&key2, &value2);
				OnInsert::on_insert(&key1, &value2);
			},
			(None, None) => {
				// noop, no hook is fired.
			},
		}
		<Map as storage::StorageMap<Key, Value>>::swap(key1, key2)
	}

	/// Store a value to be associated with the given key from the map.
	pub fn insert<KeyArg: EncodeLike<Key>>(key: KeyArg, val: Value) {
		OnInsert::on_insert(&key, &val);
		<Map as storage::StorageMap<Key, Value>>::insert(key, val)
	}

	/// Remove the value under a key.
	pub fn remove<KeyArg: EncodeLike<Key> + Clone>(key: KeyArg) {
		if let Ok(removed) = Self::try_get(key) {
			OnRemove::on_remove(&key, &removed);
		}
		<Map as storage::StorageMap<Key, Value>>::remove(key)
	}

	/// Mutate the value under a key.
	pub fn mutate<
		KeyArg: EncodeLike<Key> + Clone,
		R,
		F: FnOnce(&mut <Map as storage::StorageMap<Key, Value>>::Query) -> R,
	>(
		key: KeyArg,
		f: F,
	) -> R {
		let maybe_old_value = Self::maybe_get(key.clone());

		let result = <Map as storage::StorageMap<Key, Value>>::mutate(key.clone(), f);

		let maybe_new_value = Self::maybe_get(key.clone());
		Self::post_mutate_hooks(key, maybe_old_value, maybe_new_value);

		result
	}

	/// Mutate the item, only if an `Ok` value is returned.
	pub fn try_mutate<KeyArg, R, E, F>(key: KeyArg, f: F) -> Result<R, E>
	where
		KeyArg: EncodeLike<Key> + Clone,
		F: FnOnce(&mut <Map as storage::StorageMap<Key, Value>>::Query) -> Result<R, E>,
	{
		let maybe_old_value = Self::maybe_get(key.clone());
		let result = <Map as storage::StorageMap<Key, Value>>::try_mutate(key.clone(), f);

		if result.is_ok() {
			let maybe_new_value = Self::maybe_get(key.clone());
			Self::post_mutate_hooks(key, maybe_old_value, maybe_new_value);
		}

		result
	}

	/// Mutate the value under a key. Deletes the item if mutated to a `None`.
	pub fn mutate_exists<KeyArg: EncodeLike<Key>, R, F: FnOnce(&mut Option<Value>) -> R>(
		key: KeyArg,
		f: F,
	) -> R {
		let maybe_old_value = Self::maybe_get(key);
		let result = <Map as storage::StorageMap<Key, Value>>::mutate_exists(key, f);
		let maybe_new_value = Self::maybe_get(key);

		Self::post_mutate_hooks(key, maybe_old_value, maybe_new_value);
		result
	}

	/// Mutate the item, only if an `Ok` value is returned. Deletes the item if mutated to a `None`.
	/// `f` will always be called with an option representing if the storage item exists (`Some<V>`)
	/// or if the storage item does not exist (`None`), independent of the `QueryType`.
	pub fn try_mutate_exists<KeyArg, R, E, F>(key: KeyArg, f: F) -> Result<R, E>
	where
		KeyArg: EncodeLike<Key>,
		F: FnOnce(&mut Option<Value>) -> Result<R, E>,
	{
		let maybe_old_value = Self::maybe_get(key);
		let result = <Map as storage::StorageMap<Key, Value>>::try_mutate_exists(key, f);

		if result.is_ok() {
			let maybe_new_value = Self::maybe_get(key);
			Self::post_mutate_hooks(key, maybe_old_value, maybe_new_value);
		}

		result
	}

	/// Take the value under a key.
	pub fn take<KeyArg: EncodeLike<Key> + Clone>(
		key: KeyArg,
	) -> <Map as storage::StorageMap<Key, Value>>::Query {
		let r = <Map as storage::StorageMap<Key, Value>>::take(key);

		if let Some(removed) = Self::from_query_to_optional_value(r) {
			OnRemove::on_remove(&key, &removed);
		}

		r
	}

	/// Append the given items to the value in the storage.
	///
	/// `Value` is required to implement `codec::EncodeAppend`.
	///
	/// # Warning
	///
	/// If the storage item is not encoded properly, the storage will be overwritten
	/// and set to `[item]`. Any default value set for the storage item will be ignored
	/// on overwrite.
	pub fn append<Item, EncodeLikeItem, EncodeLikeKey>(key: EncodeLikeKey, item: EncodeLikeItem)
	where
		EncodeLikeKey: EncodeLike<Key> + Clone,
		Item: Encode,
		EncodeLikeItem: EncodeLike<Item> + Clone,
		Value: StorageAppend<Item>,
	{
		let maybe_old_value = Self::maybe_get(key.clone());
		<Map as storage::StorageMap<Key, Value>>::append(key.clone(), item);
		let maybe_new_value = Self::maybe_get(key.clone());
		Self::post_mutate_hooks(key, maybe_old_value, maybe_new_value);
	}

	/// Read the length of the storage value without decoding the entire value under the
	/// given `key`.
	///
	/// `Value` is required to implement [`StorageDecodeLength`].
	///
	/// If the value does not exists or it fails to decode the length, `None` is returned.
	/// Otherwise `Some(len)` is returned.
	///
	/// # Warning
	///
	/// `None` does not mean that `get()` does not return a value. The default value is completly
	/// ignored by this function.
	pub fn decode_len<KeyArg: EncodeLike<Key>>(key: KeyArg) -> Option<usize>
	where
		Value: StorageDecodeLength,
	{
		<Map as storage::StorageMap<Key, Value>>::decode_len(key)
	}

	/// Migrate an item with the given `key` from a defunct `OldHasher` to the current hasher.
	///
	/// If the key doesn't exist, then it's a no-op. If it does, then it returns its value.
	pub fn migrate_key<OldHasher: crate::hash::StorageHasher, KeyArg: EncodeLike<Key>>(
		key: KeyArg,
	) -> Option<Value> {
		// NOTE: does not alter value in way, thus does not emit any hook events. The underlying key
		// in storage DOES change, but the user-facing key (i.e. the KeyArg) is not changing.
		<Map as storage::StorageMap<Key, Value>>::migrate_key::<OldHasher, _>(key)
	}

	/// Iter over all value of the storage.
	///
	/// NOTE: If a value failed to decode because storage is corrupted then it is skipped.
	pub fn iter_values() -> storage::PrefixIterator<Value> {
		<Map as storage::StoragePrefixedMap<Value>>::iter_values()
	}

	/// Try and append the given item to the value in the storage.
	///
	/// Is only available if `Value` of the storage implements [`StorageTryAppend`].
	pub fn try_append<KArg, Item, EncodeLikeItem>(key: KArg, item: EncodeLikeItem) -> Result<(), ()>
	where
		KArg: EncodeLike<Key> + Clone,
		Item: Encode,
		EncodeLikeItem: EncodeLike<Item>,
		Value: StorageTryAppend<Item>,
	{
		<Map as storage::TryAppendMap<Key, Value, Item>>::try_append(key, item)
	}
}

impl<Key, Value, Map, OnRemove, OnInsert, OnUpdate>
	HookedMap<Map, Key, Value, OnRemove, OnInsert, OnUpdate>
where
	OnRemove: StorageOnRemove<Key, <Map as storage::StorageMap<Key, Value>>::Query>,
	OnInsert: StorageOnInsert<Key, Value>,
	OnUpdate: StorageOnUpdate<Key, <Map as storage::StorageMap<Key, Value>>::Query>,
	Key: FullCodec,
	Value: FullCodec,
	Map: storage::StorageMap<Key, Value>
		+ storage::generator::StorageMap<Key, Value>
		+ StoragePrefixedMap<Value>,
	<Map as storage::generator::StorageMap<Key, Value>>::Hasher:
		crate::hash::StorageHasher + crate::ReversibleStorageHasher,
{
	/// Remove all values of the storage in the overlay and up to `limit` in the backend.
	///
	/// All values in the client overlay will be deleted, if there is some `limit` then up to
	/// `limit` values are deleted from the client backend, if `limit` is none then all values in
	/// the client backend are deleted.
	///
	/// # Note
	///
	/// Calling this multiple times per block with a `limit` set leads always to the same keys being
	/// removed and the same result being returned. This happens because the keys to delete in the
	/// overlay are not taken into account when deleting keys in the backend.
	pub fn remove_all(limit: Option<u32>) -> sp_io::KillStorageResult {
		let mut removed = 0u32;
		Self::iter()
			.drain()
			.take(limit.unwrap_or(Bounded::max_value()) as usize)
			.for_each(|(k, v)| {
				OnRemove::on_remove(&k, &v);
				removed += 1;
			});

		// TODO: this one's a bit tricky.
		sp_io::KillStorageResult::AllRemoved(removed)
	}

	/// Enumerate all elements in the map in no particular order.
	///
	/// If you alter the map while doing this, you'll get undefined results.
	pub fn iter() -> storage::PrefixIterator<(Key, Value)> {
		<Map as storage::IterableStorageMap<Key, Value>>::iter()
	}

	/// Enumerate all elements in the map after a specified `starting_raw_key` in no
	/// particular order.
	///
	/// If you alter the map while doing this, you'll get undefined results.
	pub fn iter_from(starting_raw_key: Vec<u8>) -> storage::PrefixIterator<(Key, Value)> {
		<Map as storage::IterableStorageMap<Key, Value>>::iter_from(starting_raw_key)
	}

	/// Enumerate all keys in the map in no particular order.
	///
	/// If you alter the map while doing this, you'll get undefined results.
	pub fn iter_keys() -> storage::KeyPrefixIterator<Key> {
		<Map as storage::IterableStorageMap<Key, Value>>::iter_keys()
	}

	/// Enumerate all keys in the map after a specified `starting_raw_key` in no particular
	/// order.
	///
	/// If you alter the map while doing this, you'll get undefined results.
	pub fn iter_keys_from(starting_raw_key: Vec<u8>) -> storage::KeyPrefixIterator<Key> {
		<Map as storage::IterableStorageMap<Key, Value>>::iter_keys_from(starting_raw_key)
	}

	/// Remove all elements from the map and iterate through them in no particular order.
	///
	/// If you add elements to the map while doing this, you'll get undefined results.
	pub fn drain() -> storage::PrefixIterator<(Key, Value)> {
		<Map as storage::IterableStorageMap<Key, Value>>::drain()
		// TODO:
	}

	/// Translate the values of all elements by a function `f`, in the map in no particular order.
	///
	/// By returning `None` from `f` for an element, you'll remove it from the map.
	///
	/// NOTE: If a value fail to decode because storage is corrupted then it is skipped.
	pub fn translate<O: Decode, F: FnMut(Key, O) -> Option<Value>>(f: F) {
		<Map as storage::IterableStorageMap<Key, Value>>::translate(f)
		// TODO:
	}
}

impl<Key, Value, Map> StorageEntryMetadataBuilder
	for HookedMap<Map, Key, Value>
where
	Key: FullCodec + TypeInfo,
	Value: FullCodec + TypeInfo,
	Map: storage::generator::StorageMap<Key, Value>,
{

	fn build_metadata(docs: Vec<&'static str>, entries: &mut Vec<StorageEntryMetadata>) {
		let docs = if cfg!(feature = "no-metadata-docs") { vec![] } else { docs };

		let entry = StorageEntryMetadata {
			name: <Map as storage::StorageMap<Key, Value>>::pal
			modifier: <Map as storage::StorageMap<Key, Value>>::Query::METADATA,
			ty: StorageEntryType::Map {
				hashers: vec![Map::Hasher],
				key: scale_info::meta_type::<Key>(),
				value: scale_info::meta_type::<Value>(),
			},
			default: OnEmpty::get().encode(),
			docs,
		};

		entries.push(entry);
	}
}

// impl<Prefix, Hasher, Key, Value, QueryKind, OnEmpty, MaxValues> crate::traits::StorageInfoTrait
// 	for StorageMap<Prefix, Hasher, Key, Value, QueryKind, OnEmpty, MaxValues>
// where
// 	Prefix: StorageInstance,
// 	Hasher: crate::hash::StorageHasher,
// 	Key: FullCodec + MaxEncodedLen,
// 	Value: FullCodec + MaxEncodedLen,
// 	QueryKind: QueryKindTrait<Value, OnEmpty>,
// 	OnEmpty: Get<QueryKind::Query> + 'static,
// 	MaxValues: Get<Option<u32>>,
// {
// 	fn storage_info() -> Vec<StorageInfo> {
// 		vec![StorageInfo {
// 			pallet_name: Self::module_prefix().to_vec(),
// 			storage_name: Self::storage_prefix().to_vec(),
// 			prefix: Self::final_prefix().to_vec(),
// 			max_values: MaxValues::get(),
// 			max_size: Some(
// 				Hasher::max_len::<Key>()
// 					.saturating_add(Value::max_encoded_len())
// 					.saturated_into(),
// 			),
// 		}]
// 	}
// }

// /// It doesn't require to implement `MaxEncodedLen` and give no information for `max_size`.
// impl<Prefix, Hasher, Key, Value, QueryKind, OnEmpty, MaxValues>
// 	crate::traits::PartialStorageInfoTrait
// 	for StorageMap<Prefix, Hasher, Key, Value, QueryKind, OnEmpty, MaxValues>
// where
// 	Prefix: StorageInstance,
// 	Hasher: crate::hash::StorageHasher,
// 	Key: FullCodec,
// 	Value: FullCodec,
// 	QueryKind: QueryKindTrait<Value, OnEmpty>,
// 	OnEmpty: Get<QueryKind::Query> + 'static,
// 	MaxValues: Get<Option<u32>>,
// {
// 	fn partial_storage_info() -> Vec<StorageInfo> {
// 		vec![StorageInfo {
// 			pallet_name: Self::module_prefix().to_vec(),
// 			storage_name: Self::storage_prefix().to_vec(),
// 			prefix: Self::final_prefix().to_vec(),
// 			max_values: MaxValues::get(),
// 			max_size: None,
// 		}]
// 	}
// }
