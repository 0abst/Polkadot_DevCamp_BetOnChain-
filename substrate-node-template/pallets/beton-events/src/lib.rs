#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[cfg(feature = "std")]
  use serde::{Serialize, Deserialize};
  use scale_info::TypeInfo;

  use frame_support::traits::ReservableCurrency;

  use scale_info::prelude::vec::Vec;
  
  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Bet fee. (uncomment)
		//#[pallet::constant]
		//type BetFee: Get<BalanceOf<Self>>;

		/// The origin which may forcibly set or remove a name. Root can always do this.
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// The maximum length a name may be.
		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::error]// <-- Step 4. code block will replace this.
    pub enum Error<T> {
        EventAlreadyExists,
        EventDoesNotExist,
        AlreadyBet,
        DidNotBet,
        AlreadyClaimedReward,
        NotEnoughFunds,
        TooLongName,
    }

    // Types
    pub type EventName<T> = BoundedVec<u8, <T as Config>::MaxLength>;

    //pub type FirstTeamName<T> = BoundedVec<u8, <T as Config>::MaxLength>;

    //pub type SecondTeamName<T> = BoundedVec<u8, <T as Config>::MaxLength>;

    //pub type EventName = Vec<u8>;



    /*#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Bet<T: Config> {
		pub name: EventName<T>,
        pub team: u8,
	};*/


	#[pallet::storage]
    #[pallet::getter(fn beton_events_storage)]
    pub type BetOnEventsStorage<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        EventName<T>,
        u8,
        OptionQuery,
    >; 

    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		BetOnGameInitialized {
            who: T::AccountId,
            event: EventName<T>,
        },

        BetOnGameResult {
            event: EventName<T>,
            result: u8,
        },

		Bet {
            target: T::AccountId,
            event: EventName<T>,
            chosen_team: u8,
        },
		
		RewardClaimed { who: T::AccountId },
	}


	#[pallet::call]// <-- Step 6. code block will replace this. 
    impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
        pub fn initialize_bet_on_game(
            origin: OriginFor<T>,
            event: EventName<T>,
        ) -> DispatchResult {
 
            let signer = ensure_signed(origin)?;
            // error handling - add later

            ensure!(
				!<BetOnEventsStorage<T>>::contains_key(&event),
				Error::<T>::EventAlreadyExists,
			);

            <BetOnEventsStorage<T>>::insert(
                &event,
                0,
            );

			Self::deposit_event(Event::BetOnGameInitialized { who: signer, event: event });

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn set_game_result(
            origin: OriginFor<T>,
            event: EventName<T>,
            result: u8,
        ) -> DispatchResult {
 
            let signer = ensure_signed(origin)?;
            // error handling - add later

            ensure!(
				<BetOnEventsStorage<T>>::contains_key(&event),
				Error::<T>::EventDoesNotExist,
			);

            // this needs to get optimised later (I wanted to change the value, but mutate() did not work)
            <BetOnEventsStorage<T>>::remove(
                &event,
            );

            <BetOnEventsStorage<T>>::insert(
                &event,
                result,
            );

			Self::deposit_event(Event::BetOnGameResult { event: event, result });

            Ok(())
        }
    }
}
