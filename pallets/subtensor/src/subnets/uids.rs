use super::*;
use frame_support::storage::IterableStorageDoubleMap;
use sp_std::vec;
use substrate_fixed::types::I64F64;

impl<T: Config> Pallet<T> {
    /// Returns the number of filled slots on a network.
    pub fn get_subnetwork_n(netuid: u16) -> u16 {
        SubnetworkN::<T>::get(netuid)
    }

    /// Sets value for the element at the given position if it exists.
    pub fn set_element_at<N>(vec: &mut [N], position: usize, value: N) {
        if let Some(element) = vec.get_mut(position) {
            *element = value;
        }
    }

    /// Resets the trust, emission, consensus, incentive, dividends of the neuron to default
    pub fn clear_neuron(netuid: u16, neuron_uid: u16) {
        let neuron_index: usize = neuron_uid.into();
        Emission::<T>::mutate(netuid, |v| Self::set_element_at(v, neuron_index, 0));
        Trust::<T>::mutate(netuid, |v| Self::set_element_at(v, neuron_index, 0));
        Consensus::<T>::mutate(netuid, |v| Self::set_element_at(v, neuron_index, 0));
        Incentive::<T>::mutate(netuid, |v| Self::set_element_at(v, neuron_index, 0));
        Dividends::<T>::mutate(netuid, |v| Self::set_element_at(v, neuron_index, 0));
    }

    /// Replace the neuron under this uid.
    pub fn replace_neuron(
        netuid: u16,
        uid_to_replace: u16,
        new_hotkey: &T::AccountId,
        block_number: u64,
    ) {
        log::debug!(
            "replace_neuron( netuid: {:?} | uid_to_replace: {:?} | new_hotkey: {:?} ) ",
            netuid,
            uid_to_replace,
            new_hotkey
        );

        // 1. Get the old hotkey under this position.
        let old_hotkey: T::AccountId = Keys::<T>::get(netuid, uid_to_replace);

        // Do not deregister the owner's top-stake hotkey
        let mut top_stake_sn_owner_hotkey: Option<T::AccountId> = None;
        let mut max_stake_weight: I64F64 = I64F64::from_num(-1);
        for neuron_uid in 0..Self::get_subnetwork_n(netuid) {
            if let Ok(hotkey) = Self::get_hotkey_for_net_and_uid(netuid, neuron_uid) {
                let coldkey = Self::get_owning_coldkey_for_hotkey(&hotkey);
                if Self::get_subnet_owner(netuid) != coldkey {
                    continue;
                }

                let stake_weights = Self::get_stake_weights_for_hotkey_on_subnet(&hotkey, netuid);
                if stake_weights.0 > max_stake_weight {
                    max_stake_weight = stake_weights.0;
                    top_stake_sn_owner_hotkey = Some(hotkey);
                }
            }
        }

        if let Some(ref sn_owner_hotkey) = top_stake_sn_owner_hotkey {
            if sn_owner_hotkey == &old_hotkey {
                log::warn!(
                    "replace_neuron: Skipped replacement because neuron belongs to the subnet owner. \
                    And this hotkey has the highest stake weight of all the owner's hotkeys. \
                    netuid: {:?}, uid_to_replace: {:?}, new_hotkey: {:?}, owner_hotkey: {:?}",
                    netuid,
                    uid_to_replace,
                    new_hotkey,
                    sn_owner_hotkey
                );
                return;
            }
        }

        // 2. Remove previous set memberships.
        Uids::<T>::remove(netuid, old_hotkey.clone());
        IsNetworkMember::<T>::remove(old_hotkey.clone(), netuid);
        #[allow(unknown_lints)]
        Keys::<T>::remove(netuid, uid_to_replace);

        // 3. Create new set memberships.
        Self::set_active_for_uid(netuid, uid_to_replace, true); // Set to active by default.
        Keys::<T>::insert(netuid, uid_to_replace, new_hotkey.clone()); // Make hotkey - uid association.
        Uids::<T>::insert(netuid, new_hotkey.clone(), uid_to_replace); // Make uid - hotkey association.
        BlockAtRegistration::<T>::insert(netuid, uid_to_replace, block_number); // Fill block at registration.
        IsNetworkMember::<T>::insert(new_hotkey.clone(), netuid, true); // Fill network is member.

        // 4. Clear neuron certificates
        NeuronCertificates::<T>::remove(netuid, old_hotkey.clone());

        // 5. Reset new neuron's values.
        Self::clear_neuron(netuid, uid_to_replace);

        // 5a. reset axon info for the new uid.
        Axons::<T>::remove(netuid, old_hotkey);
    }

    /// Appends the uid to the network.
    pub fn append_neuron(netuid: u16, new_hotkey: &T::AccountId, block_number: u64) {
        // 1. Get the next uid. This is always equal to subnetwork_n.
        let next_uid: u16 = Self::get_subnetwork_n(netuid);
        log::debug!(
            "append_neuron( netuid: {:?} | next_uid: {:?} | new_hotkey: {:?} ) ",
            netuid,
            new_hotkey,
            next_uid
        );

        // 2. Get and increase the uid count.
        SubnetworkN::<T>::insert(netuid, next_uid.saturating_add(1));

        // 3. Expand Yuma Consensus with new position.
        Rank::<T>::mutate(netuid, |v| v.push(0));
        Trust::<T>::mutate(netuid, |v| v.push(0));
        Active::<T>::mutate(netuid, |v| v.push(true));
        Emission::<T>::mutate(netuid, |v| v.push(0));
        Consensus::<T>::mutate(netuid, |v| v.push(0));
        Incentive::<T>::mutate(netuid, |v| v.push(0));
        Dividends::<T>::mutate(netuid, |v| v.push(0));
        LastUpdate::<T>::mutate(netuid, |v| v.push(block_number));
        PruningScores::<T>::mutate(netuid, |v| v.push(0));
        ValidatorTrust::<T>::mutate(netuid, |v| v.push(0));
        ValidatorPermit::<T>::mutate(netuid, |v| v.push(false));

        // 4. Insert new account information.
        Keys::<T>::insert(netuid, next_uid, new_hotkey.clone()); // Make hotkey - uid association.
        Uids::<T>::insert(netuid, new_hotkey.clone(), next_uid); // Make uid - hotkey association.
        BlockAtRegistration::<T>::insert(netuid, next_uid, block_number); // Fill block at registration.
        IsNetworkMember::<T>::insert(new_hotkey.clone(), netuid, true); // Fill network is member.
    }

    /// Returns true if the uid is set on the network.
    ///
    pub fn is_uid_exist_on_network(netuid: u16, uid: u16) -> bool {
        Keys::<T>::contains_key(netuid, uid)
    }

    /// Returns true if the hotkey holds a slot on the network.
    ///
    pub fn is_hotkey_registered_on_network(netuid: u16, hotkey: &T::AccountId) -> bool {
        Uids::<T>::contains_key(netuid, hotkey)
    }

    /// Returs the hotkey under the network uid as a Result. Ok if the uid is taken.
    ///
    pub fn get_hotkey_for_net_and_uid(
        netuid: u16,
        neuron_uid: u16,
    ) -> Result<T::AccountId, DispatchError> {
        Keys::<T>::try_get(netuid, neuron_uid)
            .map_err(|_err| Error::<T>::HotKeyNotRegisteredInSubNet.into())
    }

    /// Returns the uid of the hotkey in the network as a Result. Ok if the hotkey has a slot.
    ///
    pub fn get_uid_for_net_and_hotkey(
        netuid: u16,
        hotkey: &T::AccountId,
    ) -> Result<u16, DispatchError> {
        Uids::<T>::try_get(netuid, hotkey)
            .map_err(|_err| Error::<T>::HotKeyNotRegisteredInSubNet.into())
    }

    /// Returns the stake of the uid on network or 0 if it doesnt exist.
    ///
    pub fn get_stake_for_uid_and_subnetwork(netuid: u16, neuron_uid: u16) -> u64 {
        if let Ok(hotkey) = Self::get_hotkey_for_net_and_uid(netuid, neuron_uid) {
            Self::get_stake_for_hotkey_on_subnet(&hotkey, netuid)
        } else {
            0
        }
    }

    /// Return a list of all networks a hotkey is registered on.
    ///
    pub fn get_registered_networks_for_hotkey(hotkey: &T::AccountId) -> Vec<u16> {
        let mut all_networks: Vec<u16> = vec![];
        for (network, is_registered) in
            <IsNetworkMember<T> as IterableStorageDoubleMap<T::AccountId, u16, bool>>::iter_prefix(
                hotkey,
            )
        {
            if is_registered {
                all_networks.push(network)
            }
        }
        all_networks
    }

    /// Return true if a hotkey is registered on any network.
    ///
    pub fn is_hotkey_registered_on_any_network(hotkey: &T::AccountId) -> bool {
        for (_, is_registered) in
            <IsNetworkMember<T> as IterableStorageDoubleMap<T::AccountId, u16, bool>>::iter_prefix(
                hotkey,
            )
        {
            if is_registered {
                return true;
            }
        }
        false
    }
}
