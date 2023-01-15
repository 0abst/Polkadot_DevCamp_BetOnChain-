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

  use frame_support::traits::{ExistenceRequirement, ReservableCurrency, Currency};

  use scale_info::prelude::vec::Vec;

  use pallet_balances;
  use beton_events;
  use pallet_treasury;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

    #[pallet::config]
	pub trait Config: frame_system::Config + beton_events::Config + pallet_treasury::Config {
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

	#[pallet::error]
    pub enum Error<T> {
        // Event already exists
        EventAlreadyExists,
        // event does not exist
        EventDoesNotExist,
        // event already ended
        EventAlreadyEnded,
        // event has not ended yet
        EventHasNotEndedYet,
        // can not bet on team zero
        CanNotBetOnZero,
        // you have already bet
        AlreadyBet,
        // did not bet
        DidNotBet,
        // already claimed reward
        AlreadyClaimedReward,
        // not enough funds
        NotEnoughFunds,
    }
    
    // types
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct BetOnSomething<T: Config> {
		pub team: u8,
        pub amount: BalanceOf<T>,
	}

    pub type BalanceOf<T = ()> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::storage]
    pub type BetOnStorage<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        beton_events::EventName<T>,
        BetOnSomething<T>, // bet
        OptionQuery,
    >;

    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Bet {
            who: T::AccountId,
            amount: BalanceOf<T>,
            treasury_id: T::AccountId,
        },

        BetLost {
            who: T::AccountId,
        },
		
		RewardClaimed {
            who: T::AccountId,
            amount: BalanceOf<T>,
        },

        BetRemoved {
            who: T::AccountId,
        }
	}

	#[pallet::call]
    impl<T: Config> Pallet<T> {

        /**
        * Place a bet
        * - This can be called only once at a time
        */
		#[pallet::weight(0)]
        pub fn bet(
            origin: OriginFor<T>,
            event: beton_events::EventName<T>,
            team: u8,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
 
            let signer = ensure_signed(origin)?;
            // error handling - add later

            ensure!(
				<beton_events::BetOnEventsStorage<T>>::contains_key(&event),
				Error::<T>::EventDoesNotExist,
			);

            ensure!(
				<beton_events::BetOnEventsStorage<T>>::get(&event) == Some(0),
				Error::<T>::EventAlreadyEnded,
			);

            ensure!(
				team != 0,
				Error::<T>::CanNotBetOnZero,
			);

            ensure!(
                !<BetOnStorage<T>>::contains_key(&signer, &event),
                Error::<T>::AlreadyBet,
            );

            // Send Money from better to treasury
            // amount

           let treasury_id = pallet_treasury::Pallet::<T>::account_id();

           <<T as Config>::Currency as Currency<T::AccountId>>::transfer(
                &signer,
                &pallet_treasury::Pallet::<T>::account_id(),
                amount,
                ExistenceRequirement::AllowDeath,
            );

            let bet_on_something = BetOnSomething {
                team,
                amount,
            };

            <BetOnStorage<T>>::set(&signer, &event, Some(bet_on_something));

			Self::deposit_event(Event::Bet { who: signer, amount, treasury_id });

            Ok(())
        }

        /**
        * Claim the reward if your bet was successful
        * - Removes the bet from BetOnStorage
        */
        #[pallet::weight(0)]
        pub fn claim_reward(
            origin: OriginFor<T>,
            event: beton_events::EventName<T>,
        ) -> DispatchResult {
 
            let signer = ensure_signed(origin)?;
            // error handling - add later

            ensure!(
                <BetOnStorage<T>>::contains_key(&signer, &event),
                Error::<T>::DidNotBet,
            );


            ensure!(
				<beton_events::BetOnEventsStorage<T>>::contains_key(&event),
                Error::<T>::EventDoesNotExist,
			);

            ensure!(
				<beton_events::BetOnEventsStorage<T>>::get(&event) != Some(0),
				Error::<T>::EventHasNotEndedYet,
			);

            let bet = <BetOnStorage<T>>::take(&signer, &event).unwrap();

            if <beton_events::BetOnEventsStorage<T>>::get(&event) != Some(bet.team) {
                Self::deposit_event(Event::BetLost { who: signer });

                return Ok(())
            }

            // Won the bet

            // Send Money from treasury to better
            let treasury_id = pallet_treasury::Pallet::<T>::account_id();

            <<T as Config>::Currency as Currency<T::AccountId>>::transfer(
                &pallet_treasury::Pallet::<T>::account_id(),
                 &signer,
                 bet.amount + bet.amount, // bet.amount * 2
                 ExistenceRequirement::AllowDeath,
             );
            

			Self::deposit_event(Event::RewardClaimed { who: signer, amount: bet.amount + bet.amount });

            Ok(())
        }

        /**
        * Claim the reward if your bet was successful
        */
        #[pallet::weight(0)]
        pub fn remove_bet(
            origin: OriginFor<T>,
            event: beton_events::EventName<T>,
        ) -> DispatchResult {
 
            let signer = ensure_signed(origin)?;
            // error handling - add later

            ensure!(
                <BetOnStorage<T>>::contains_key(&signer, &event),
                Error::<T>::DidNotBet,
            );

            <BetOnStorage<T>>::remove(&signer, &event);

			Self::deposit_event(Event::BetRemoved { who: signer });

            Ok(())
        }
    }
}