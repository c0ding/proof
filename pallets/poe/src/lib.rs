#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
};
use frame_system::ensure_signed;
use sp_std::prelude::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as PoeModule {
        //存证
        Proofs get(fn proofs):map hasher(blake2_128_concat) Vec<u8>=>(T::AccountId,T::BlockNumber);



    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        BlockNumber = <T as frame_system::Trait>::BlockNumber,
    {
        ClaimNew(AccountId, Vec<u8>),
        ClaimRemove(AccountId, Vec<u8>, BlockNumber),
        ClaimTransact(AccountId, Vec<u8>, AccountId, BlockNumber),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        ClaimExisted,
        ClaimNotExist,
        ClaimNotOwner,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;
        #[weight = 100]
        pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {

            let sender = ensure_signed(origin)?;
            Self::create(sender.clone(), claim)
        }

        #[weight = 100]
        pub fn remove_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::remove(sender.clone(), claim)
        }

        #[weight = 100]
        pub fn transact_claim(origin, claim: Vec<u8>,dest:T::AccountId)-> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::transact(sender.clone(),claim,dest.clone())
        }


    }
}

impl<T: Trait> Module<T> {
    fn create(sender: T::AccountId, claim: Vec<u8>) -> dispatch::DispatchResult {
        //判断存证是否已经存在，
        ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ClaimExisted);
        //存储存证
        Proofs::<T>::insert(
            &claim,
            (sender.clone(), frame_system::Module::<T>::block_number()),
        );
        //事件通知
        Self::deposit_event(RawEvent::ClaimNew(sender.clone(), claim));

        Ok(())
    }

    fn remove(sender: T::AccountId, claim: Vec<u8>) -> dispatch::DispatchResult {
        ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

        let (owner, block_number) = Self::proofs(claim.clone());

        ensure!(sender == owner, Error::<T>::ClaimNotOwner);

        Proofs::<T>::remove(claim.clone());

        Self::deposit_event(RawEvent::ClaimRemove(sender.clone(), claim, block_number));

        Ok(())
    }

    fn transact(
        sender: T::AccountId,
        claim: Vec<u8>,
        dest: T::AccountId,
    ) -> dispatch::DispatchResult {
        ensure!(
            Proofs::<T>::contains_key(claim.clone()),
            Error::<T>::ClaimNotExist
        );
        //判断存证是否存在
        ensure!(
            Proofs::<T>::contains_key(claim.clone()),
            Error::<T>::ClaimNotExist
        );

        let (owner, block_number) = Proofs::<T>::get(claim.clone());
        //判断存证是否属于发送者
        ensure!(owner == sender, Error::<T>::ClaimNotOwner);

        //当前accountid 移除 存证，目标accountid 增加 存证
        //直接更改claim 的 value
        Proofs::<T>::insert(&claim, (dest.clone(), block_number));

        Self::deposit_event(RawEvent::ClaimTransact(
            sender.clone(),
            claim,
            dest.clone(),
            block_number,
        ));

        Ok(())
    }
}
