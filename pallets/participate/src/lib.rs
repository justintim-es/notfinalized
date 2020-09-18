#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, ensure, decl_storage, decl_event, decl_error, dispatch, traits::Get, weights::{DispatchClass, Pays, Weight}};
use frame_system::ensure_signed;
use frame_system::ensure_root;
use sp_std::vec::Vec;
use sp_runtime::traits::OpaqueKeys;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait + pallet_session::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
		Humanify: map hasher(blake2_128_concat) Vec<u8> => bool;
        Peers: map hasher(blake2_128_concat) Vec<u8> => Option<T::AccountId>;
        Validators config(initial_validators): Vec<T::AccountId>
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		Humanify,
		Peer,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

		#[weight = (100_000, DispatchClass::Normal, Pays::No)]
		pub fn insert_humanify(origin, random: Vec<u8>) -> dispatch::DispatchResult {
			let _ = ensure_root(origin)?;
			Humanify::insert(random, true);
			Ok(())
        }
		#[weight = (100_000, DispatchClass::Normal, Pays::No)]
		pub fn confirm_humanify(origin, random: Vec<u8>, peer_id: Vec<u8>, account: T::AccountId) -> dispatch::DispatchResult {
			let _ = ensure_root(origin)?;
			ensure!(Humanify::get(random), Error::<T>::Humanify);
			if let None = <Peers<T>>::get(&peer_id) {
                <Peers<T>>::insert(peer_id, &account);
                let mut validators = <Validators<T>>::get();
                validators.push(account);
                <Validators<T>>::put(validators);
			}
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}
impl<T: Trait> pallet_session::ShouldEndSession<T::BlockNumber> for Module<T> {
    fn should_end_session(now: T::BlockNumber) -> bool {
        true
    }
}
impl<T: Trait> pallet_session::SessionManager<T::AccountId> for Module<T> {
    fn new_session(new_index: u32) -> Option<Vec<T::AccountId>> {
        Some(<Validators<T>>::get())
    }
    fn end_session(end_index: u32) {
        
    }
    fn start_session(start_index: u32) {

    }
}
impl<T: Trait> frame_support::traits::EstimateNextSessionRotation<T::BlockNumber> for Module<T> {
    fn estimate_next_session_rotation(now: T::BlockNumber) -> Option<T::BlockNumber> {
        Some(now)
    }
    fn weight(now: T::BlockNumber) -> Weight {
        0
    }
}
