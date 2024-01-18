#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen)]
pub enum Clubs {
	Empty,
	CoolGuys,
	EvenCoolerGuys,
	BadGuys,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use crate::Clubs;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn club_list)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub(super) type ClubList<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Clubs>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClubAssigned(T::AccountId, Clubs),
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Assign a club to a user or remove it.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn assign_club(
			origin: OriginFor<T>,
			user: T::AccountId,
			club: Clubs,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Remove from storage instead of storing as "Empty" club
			if let Clubs::Empty = club {
				ClubList::<T>::remove(user.clone());
			} else {
				ClubList::<T>::insert(user.clone(), club);
			}

			Self::deposit_event(Event::ClubAssigned(user, club));

			Ok(())
		}
	}
}
