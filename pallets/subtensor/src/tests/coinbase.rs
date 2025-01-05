#![allow(unused, clippy::indexing_slicing, clippy::panic, clippy::unwrap_used)]
use super::mock::*;

use frame_support::assert_ok;
use sp_core::U256;
use substrate_fixed::types::I64F64;

// To run this test specifically, use the following command:
// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test coinbase test_coinbase_basic -- --nocapture
#[test]

fn test_coinbase_basic() {
    new_test_ext(1).execute_with(|| {
        assert!(false);

        // // Define network ID
        // let netuid: u16 = 1;
        // let hotkey = U256::from(0);
        // let coldkey = U256::from(3);

        // // Create a network with a tempo 1
        // add_network(netuid, 1, 0);
        // register_ok_neuron(netuid, hotkey, coldkey, 100000);
        // SubtensorModule::create_account_if_non_existent(&coldkey, &hotkey);
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&coldkey, &hotkey, 1000);

        // // Set the subnet emission value to 1.
        // SubtensorModule::set_emission_values(&[netuid], vec![1]).unwrap();
        // assert_eq!(SubtensorModule::get_subnet_emission_value(netuid), 1);

        // // Hotkey has no pending emission
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);

        // // Hotkey has same stake
        // assert_eq!(SubtensorModule::get_total_stake_for_hotkey(&hotkey), 1000);

        // // Subnet has no pending emission.
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);

        // // Step block
        // next_block();

        // // Hotkey has no pending emission
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);

        // // Hotkey has same stake
        // assert_eq!(SubtensorModule::get_total_stake_for_hotkey(&hotkey), 1000);

        // // Subnet has no pending emission of 1 ( from coinbase )
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 1);

        // // Step block releases
        // next_block();

        // // Subnet pending has been drained.
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);

        // // Hotkey pending immediately drained.
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);

        // // Hotkey has NEW stake
        // assert_eq!(
        //     SubtensorModule::get_total_stake_for_hotkey(&hotkey),
        //     1000 + 2
        // );

        // // Set the hotkey drain time to 2 block.
        // SubtensorModule::set_hotkey_emission_tempo(2);

        // // Step block releases
        // next_block();

        // // Subnet pending increased by 1
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 1);

        // // Hotkey pending not increased (still on subnet)
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);

        // // Hotkey has same stake
        // assert_eq!(
        //     SubtensorModule::get_total_stake_for_hotkey(&hotkey),
        //     1000 + 2
        // );

        // // Step block releases
        // next_block();

        // // Subnet pending has been drained.
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);

        // // Hotkey pending drained.
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);

        // // Hotkey has 2 new TAO.
        // assert_eq!(
        //     SubtensorModule::get_total_stake_for_hotkey(&hotkey),
        //     1000 + 4
        // );
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --package pallet-subtensor --test coinbase test_coinbase_nominator_drainage_overflow -- --nocapture
#[test]
fn test_coinbase_nominator_drainage_overflow() {
    new_test_ext(1).execute_with(|| {
        assert!(false);

        // // 1. Set up the network and accounts
        // let netuid: u16 = 1;
        // let hotkey = U256::from(0);
        // let coldkey = U256::from(3);
        // let nominator1 = U256::from(1);
        // let nominator2 = U256::from(2);

        // log::debug!("Setting up network with netuid: {}", netuid);
        // log::debug!("Hotkey: {:?}, Coldkey: {:?}", hotkey, coldkey);
        // log::debug!("Nominators: {:?}, {:?}", nominator1, nominator2);

        // // 2. Create network and register neuron
        // add_network(netuid, 1, 0);
        // register_ok_neuron(netuid, hotkey, coldkey, 100000);
        // SubtensorModule::create_account_if_non_existent(&coldkey, &hotkey);

        // log::debug!("Network created and neuron registered");

        // // 3. Set up balances and stakes
        // SubtensorModule::add_balance_to_coldkey_account(&coldkey, 1000);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator1, 1500);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator2, 1500);

        // log::debug!("Balances added to accounts");

        // // 4. Make the hotkey a delegate
        // let vali_take = (u16::MAX as u64 / 10);
        // assert_ok!(SubtensorModule::do_become_delegate(
        //     RuntimeOrigin::signed(coldkey),
        //     hotkey,
        //     vali_take as u16
        // ));

        // log::debug!("Hotkey became a delegate with minimum take");

        // // Add stakes for nominators
        // // Add the stake directly to their coldkey-hotkey account
        // // This bypasses the accounting in stake delta
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator1, &hotkey, 5e9 as u64);
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator2, &hotkey, 5e9 as u64);
        // let initial_stake = 5e9 as u64;

        // // Log the stakes for hotkey, nominator1, and nominator2
        // log::debug!(
        //     "Initial stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}",
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey)
        // );
        // log::debug!("Stakes added for nominators");

        // // 5. Set emission and verify initial states
        // let to_emit = 20_000e9 as u64;
        // SubtensorModule::set_emission_values(&[netuid], vec![to_emit]).unwrap();
        // assert_eq!(SubtensorModule::get_subnet_emission_value(netuid), to_emit);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // assert_eq!(
        //     SubtensorModule::get_total_stake_for_hotkey(&hotkey),
        //     initial_stake * 2
        // );
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);

        // log::debug!("Emission set and initial states verified");

        // // 6. Set hotkey emission tempo
        // SubtensorModule::set_hotkey_emission_tempo(1);
        // log::debug!("Hotkey emission tempo set to 1");

        // // 7. Simulate blocks and check emissions
        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), to_emit);
        // log::debug!(
        //     "After first block, pending emission: {}",
        //     SubtensorModule::get_pending_emission(netuid)
        // );

        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // log::debug!("After second block, pending emission drained");

        // // 8. Check final stakes
        // let hotkey_stake = SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey);
        // let nominator1_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // let nominator2_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey);

        // log::debug!(
        //     "Final stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}",
        //     hotkey_stake,
        //     nominator1_stake,
        //     nominator2_stake
        // );

        // // 9. Verify distribution
        // let total_emission = to_emit * 2; // to_emit per block for 2 blocks
        // let hotkey_emission = (I64F64::from_num(total_emission) / I64F64::from_num(u16::MAX)
        //     * I64F64::from_num(vali_take))
        // .to_num::<u64>();
        // let remaining_emission = total_emission - hotkey_emission;
        // let nominator_emission = remaining_emission / 2;

        // log::debug!(
        //     "Calculated emissions - Hotkey: {}, Each Nominator: {}",
        //     hotkey_emission,
        //     nominator_emission
        // );

        // // Debug: Print the actual stakes
        // log::debug!("Actual hotkey stake: {}", hotkey_stake);
        // log::debug!("Actual nominator1 stake: {}", nominator1_stake);
        // log::debug!("Actual nominator2 stake: {}", nominator2_stake);

        // // Debug: Check the total stake for the hotkey
        // let total_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // log::debug!("Total stake for hotkey: {}", total_stake);

        // // Assertions
        // let expected_hotkey_stake = 4_000e9 as u64;
        // let eps = 0.5e9 as u64;
        // assert!(
        //     hotkey_stake >= expected_hotkey_stake - eps
        //         && hotkey_stake <= expected_hotkey_stake + eps,
        //     "Hotkey stake mismatch - expected: {}, actual: {}",
        //     expected_hotkey_stake,
        //     hotkey_stake
        // );
        // assert_eq!(
        //     nominator1_stake,
        //     initial_stake + nominator_emission,
        //     "Nominator1 stake mismatch"
        // );
        // assert_eq!(
        //     nominator2_stake,
        //     initial_stake + nominator_emission,
        //     "Nominator2 stake mismatch"
        // );

        // // 10. Check total stake
        // assert_eq!(
        //     total_stake,
        //     initial_stake + initial_stake + total_emission,
        //     "Total stake mismatch"
        // );

        // log::debug!("Test completed");
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --package pallet-subtensor --test coinbase test_coinbase_nominator_drainage_no_deltas -- --nocapture
#[test]
fn test_coinbase_nominator_drainage_no_deltas() {
    new_test_ext(1).execute_with(|| {
        assert!(false);

        // // 1. Set up the network and accounts
        // let netuid: u16 = 1;
        // let hotkey = U256::from(0);
        // let coldkey = U256::from(3);
        // let nominator1 = U256::from(1);
        // let nominator2 = U256::from(2);

        // log::debug!("Setting up network with netuid: {}", netuid);
        // log::debug!("Hotkey: {:?}, Coldkey: {:?}", hotkey, coldkey);
        // log::debug!("Nominators: {:?}, {:?}", nominator1, nominator2);

        // // 2. Create network and register neuron
        // add_network(netuid, 1, 0);
        // register_ok_neuron(netuid, hotkey, coldkey, 100000);
        // SubtensorModule::create_account_if_non_existent(&coldkey, &hotkey);

        // log::debug!("Network created and neuron registered");

        // // 3. Set up balances and stakes
        // SubtensorModule::add_balance_to_coldkey_account(&coldkey, 1000);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator1, 1500);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator2, 1500);

        // log::debug!("Balances added to accounts");

        // // 4. Make the hotkey a delegate
        // let val_take = (u16::MAX as u64 / 10);
        // assert_ok!(SubtensorModule::do_become_delegate(
        //     RuntimeOrigin::signed(coldkey),
        //     hotkey,
        //     val_take as u16
        // ));

        // log::debug!("Hotkey became a delegate with minimum take");

        // // Add stakes for nominators
        // // Add the stake directly to their coldkey-hotkey account
        // // This bypasses the accounting in stake delta
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator1, &hotkey, 100);
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator2, &hotkey, 100);

        // // Log the stakes for hotkey, nominator1, and nominator2
        // log::debug!(
        //     "Initial stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}",
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey)
        // );
        // log::debug!("Stakes added for nominators");

        // // 5. Set emission and verify initial states
        // SubtensorModule::set_emission_values(&[netuid], vec![10]).unwrap();
        // assert_eq!(SubtensorModule::get_subnet_emission_value(netuid), 10);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // assert_eq!(SubtensorModule::get_total_stake_for_hotkey(&hotkey), 200);
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);

        // log::debug!("Emission set and initial states verified");

        // // 6. Set hotkey emission tempo
        // SubtensorModule::set_hotkey_emission_tempo(1);
        // log::debug!("Hotkey emission tempo set to 1");

        // // 7. Simulate blocks and check emissions
        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 10);
        // log::debug!(
        //     "After first block, pending emission: {}",
        //     SubtensorModule::get_pending_emission(netuid)
        // );

        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // log::debug!("After second block, pending emission drained");

        // // 8. Check final stakes
        // let hotkey_stake = SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey);
        // let nominator1_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // let nominator2_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey);

        // log::debug!(
        //     "Final stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}",
        //     hotkey_stake,
        //     nominator1_stake,
        //     nominator2_stake
        // );

        // // 9. Verify distribution
        // let min_take = val_take;
        // let total_emission = 20; // 10 per block for 2 blocks
        // let hotkey_emission = total_emission * min_take / u16::MAX as u64;
        // let remaining_emission = total_emission - hotkey_emission;
        // let nominator_emission = remaining_emission / 2;

        // log::debug!(
        //     "Calculated emissions - Hotkey: {}, Each Nominator: {}",
        //     hotkey_emission,
        //     nominator_emission
        // );

        // // Debug: Print the actual stakes
        // log::debug!("Actual hotkey stake: {}", hotkey_stake);
        // log::debug!("Actual nominator1 stake: {}", nominator1_stake);
        // log::debug!("Actual nominator2 stake: {}", nominator2_stake);

        // // Debug: Check the total stake for the hotkey
        // let total_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // log::debug!("Total stake for hotkey: {}", total_stake);

        // // Assertions
        // assert_eq!(hotkey_stake, 2, "Hotkey stake mismatch");
        // assert_eq!(
        //     nominator1_stake,
        //     100 + nominator_emission,
        //     "Nominator1 stake mismatch"
        // );
        // assert_eq!(
        //     nominator2_stake,
        //     100 + nominator_emission,
        //     "Nominator2 stake mismatch"
        // );

        // // 10. Check total stake
        // assert_eq!(total_stake, 200 + total_emission, "Total stake mismatch");

        // log::debug!("Test completed");
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --package pallet-subtensor --test coinbase test_coinbase_nominator_drainage_with_positive_delta -- --nocapture
#[test]
fn test_coinbase_nominator_drainage_with_positive_delta() {
    new_test_ext(1).execute_with(|| {
        assert!(false);

        // // 1. Set up the network and accounts
        // let netuid: u16 = 1;
        // let hotkey = U256::from(0);
        // let coldkey = U256::from(3);
        // let nominator1 = U256::from(1);
        // let nominator2 = U256::from(2);

        // log::debug!("Setting up network with netuid: {}", netuid);
        // log::debug!("Hotkey: {:?}, Coldkey: {:?}", hotkey, coldkey);
        // log::debug!("Nominators: {:?}, {:?}", nominator1, nominator2);

        // // 2. Create network and register neuron
        // add_network(netuid, 1, 0);
        // register_ok_neuron(netuid, hotkey, coldkey, 100000);
        // SubtensorModule::create_account_if_non_existent(&coldkey, &hotkey);

        // log::debug!("Network created and neuron registered");

        // // 3. Set up balances and stakes
        // SubtensorModule::add_balance_to_coldkey_account(&coldkey, 1000);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator1, 1500);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator2, 1500);

        // log::debug!("Balances added to accounts");

        // // 4. Make the hotkey a delegate
        // let val_take = (u16::MAX as u64 / 10);
        // assert_ok!(SubtensorModule::do_become_delegate(
        //     RuntimeOrigin::signed(coldkey),
        //     hotkey,
        //     val_take as u16
        // ));

        // log::debug!("Hotkey became a delegate with minimum take");

        // // Add stakes for nominators
        // // Add the stake directly to their coldkey-hotkey account
        // // This bypasses the accounting in stake delta
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator1, &hotkey, 100);
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator2, &hotkey, 100);

        // // Do an add_stake for nominator 1
        // assert_ok!(SubtensorModule::do_add_stake(
        //     RuntimeOrigin::signed(nominator1),
        //     hotkey,
        //     123
        // )); // We should not expect this to impact the emissions

        // // Log the stakes for hotkey, nominator1, and nominator2
        // log::debug!(
        //     "Initial stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}",
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey)
        // );
        // log::debug!("Stakes added for nominators");

        // let nominator1_stake_before =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // assert_eq!(nominator1_stake_before, 100 + 123); // The stake should include the added stake

        // // 5. Set emission and verify initial states
        // SubtensorModule::set_emission_values(&[netuid], vec![10]).unwrap();
        // assert_eq!(SubtensorModule::get_subnet_emission_value(netuid), 10);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // assert_eq!(
        //     SubtensorModule::get_total_stake_for_hotkey(&hotkey),
        //     200 + 123
        // );
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);

        // log::debug!("Emission set and initial states verified");

        // // 6. Set hotkey emission tempo
        // SubtensorModule::set_hotkey_emission_tempo(1);
        // log::debug!("Hotkey emission tempo set to 1");

        // // 7. Simulate blocks and check emissions
        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 10);
        // log::debug!(
        //     "After first block, pending emission: {}",
        //     SubtensorModule::get_pending_emission(netuid)
        // );

        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // log::debug!("After second block, pending emission drained");

        // // 8. Check final stakes
        // let hotkey_stake = SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey);
        // let nominator1_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // let nominator2_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey);

        // log::debug!(
        //     "Final stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}",
        //     hotkey_stake,
        //     nominator1_stake,
        //     nominator2_stake
        // );

        // // 9. Verify distribution
        // let min_take = val_take;
        // let total_emission = 20; // 10 per block for 2 blocks
        // let hotkey_emission = total_emission * min_take / u16::MAX as u64;
        // let remaining_emission = total_emission - hotkey_emission;
        // let nominator_emission = remaining_emission / 2;
        // // Notice that nominator emission is equal for both nominators, even though nominator1 added stake

        // log::debug!(
        //     "Calculated emissions - Hotkey: {}, Each Nominator: {}",
        //     hotkey_emission,
        //     nominator_emission
        // );

        // // Debug: Print the actual stakes
        // log::debug!("Actual hotkey stake: {}", hotkey_stake);
        // log::debug!("Actual nominator1 stake: {}", nominator1_stake);
        // log::debug!("Actual nominator2 stake: {}", nominator2_stake);

        // // Debug: Check the total stake for the hotkey
        // let total_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // log::debug!("Total stake for hotkey: {}", total_stake);

        // // Assertions
        // assert_eq!(hotkey_stake, 2, "Hotkey stake mismatch");
        // assert_eq!(
        //     nominator1_stake,
        //     100 + 123 + nominator_emission,
        //     "Nominator1 stake mismatch"
        // );
        // assert_eq!(
        //     nominator2_stake,
        //     100 + nominator_emission,
        //     "Nominator2 stake mismatch"
        // );

        // // 10. Check total stake
        // // Includes the added stake from nominator1
        // assert_eq!(
        //     total_stake,
        //     200 + 123 + total_emission,
        //     "Total stake mismatch"
        // );

        // log::debug!("Test completed");
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --package pallet-subtensor --test coinbase test_coinbase_nominator_drainage_with_negative_delta -- --nocapture
#[test]
fn test_coinbase_nominator_drainage_with_negative_delta() {
    new_test_ext(1).execute_with(|| {
        assert!(false);

        // // 1. Set up the network and accounts
        // let netuid: u16 = 1;
        // let hotkey = U256::from(0);
        // let coldkey = U256::from(3);
        // let nominator1 = U256::from(1);
        // let nominator2 = U256::from(2);

        // log::debug!("Setting up network with netuid: {}", netuid);
        // log::debug!("Hotkey: {:?}, Coldkey: {:?}", hotkey, coldkey);
        // log::debug!("Nominators: {:?}, {:?}", nominator1, nominator2);

        // // 2. Create network and register neuron
        // add_network(netuid, 1, 0);
        // register_ok_neuron(netuid, hotkey, coldkey, 100000);
        // SubtensorModule::create_account_if_non_existent(&coldkey, &hotkey);

        // log::debug!("Network created and neuron registered");

        // // 3. Set up balances and stakes
        // SubtensorModule::add_balance_to_coldkey_account(&coldkey, 1000);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator1, 1500);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator2, 1500);

        // log::debug!("Balances added to accounts");

        // // 4. Make the hotkey a delegate
        // let val_take = (u16::MAX as u64 / 10);
        // assert_ok!(SubtensorModule::do_become_delegate(
        //     RuntimeOrigin::signed(coldkey),
        //     hotkey,
        //     val_take as u16
        // ));

        // log::debug!("Hotkey became a delegate with minimum take");

        // // Add stakes for nominators
        // // Add the stake directly to their coldkey-hotkey account
        // // This bypasses the accounting in stake delta
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator1, &hotkey, 100);
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator2, &hotkey, 100);

        // // Do an remove_stake for nominator 1
        // assert_ok!(SubtensorModule::remove_stake(
        //     RuntimeOrigin::signed(nominator1),
        //     hotkey,
        //     12
        // )); // We should expect the emissions to be impacted;
        //     // The viable stake should be the *new* stake for nominator 1

        // // Log the stakes for hotkey, nominator1, and nominator2
        // log::debug!(
        //     "Initial stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}",
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey)
        // );
        // log::debug!("Stakes added for nominators");

        // let nominator_1_stake_before =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // // Notice that nominator1 stake is the new stake, including the removed stake
        // assert_eq!(nominator_1_stake_before, 100 - 12);

        // // 5. Set emission and verify initial states
        // SubtensorModule::set_emission_values(&[netuid], vec![10]).unwrap();
        // assert_eq!(SubtensorModule::get_subnet_emission_value(netuid), 10);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // assert_eq!(
        //     SubtensorModule::get_total_stake_for_hotkey(&hotkey),
        //     200 - 12
        // );
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);

        // log::debug!("Emission set and initial states verified");

        // // 6. Set hotkey emission tempo
        // SubtensorModule::set_hotkey_emission_tempo(1);
        // log::debug!("Hotkey emission tempo set to 1");

        // // 7. Simulate blocks and check emissions
        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 10);
        // log::debug!(
        //     "After first block, pending emission: {}",
        //     SubtensorModule::get_pending_emission(netuid)
        // );

        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // log::debug!("After second block, pending emission drained");

        // // 8. Check final stakes
        // let delegate_stake = SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey);
        // let total_hotkey_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // let nominator1_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // let nominator2_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey);

        // log::debug!(
        //     "Final stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}, Total Hotkey Stake: {}",
        //     delegate_stake,
        //     nominator1_stake,
        //     nominator2_stake,
        //     total_hotkey_stake
        // );

        // // 9. Verify distribution
        // let min_take = val_take;
        // let total_emission = 20; // 10 per block for 2 blocks
        // let hotkey_emission = total_emission * min_take / u16::MAX as u64;
        // let remaining_emission = total_emission - hotkey_emission;

        // let nominator_1_emission = remaining_emission * nominator1_stake / total_hotkey_stake;
        // let nominator_2_emission = remaining_emission * nominator2_stake / total_hotkey_stake;

        // log::debug!(
        //     "Calculated emissions - Hotkey: {}, Each Nominator: 1;{}, 2;{}",
        //     hotkey_emission,
        //     nominator_1_emission,
        //     nominator_2_emission
        // );

        // // Debug: Print the actual stakes
        // log::debug!("Actual hotkey stake: {}", delegate_stake);
        // log::debug!("Actual nominator1 stake: {}", nominator1_stake);
        // log::debug!("Actual nominator2 stake: {}", nominator2_stake);

        // // Debug: Check the total stake for the hotkey
        // let total_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // log::debug!("Total stake for hotkey: {}", total_stake);

        // // Assertions
        // assert_eq!(delegate_stake, 2, "Hotkey stake mismatch");
        // assert_eq!(
        //     nominator1_stake,
        //     100 - 12 + nominator_1_emission,
        //     "Nominator1 stake mismatch"
        // );
        // assert_eq!(
        //     nominator2_stake,
        //     100 + nominator_2_emission,
        //     "Nominator2 stake mismatch"
        // );

        // // 10. Check total stake
        // assert_eq!(
        //     total_stake,
        //     200 - 12 + total_emission,
        //     "Total stake mismatch"
        // );

        // log::debug!("Test completed");
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --package pallet-subtensor --test coinbase test_coinbase_nominator_drainage_with_neutral_delta -- --nocapture
#[test]
fn test_coinbase_nominator_drainage_with_neutral_delta() {
    new_test_ext(1).execute_with(|| {
        assert!(false);

        // // 1. Set up the network and accounts
        // let netuid: u16 = 1;
        // let hotkey = U256::from(0);
        // let coldkey = U256::from(3);
        // let nominator1 = U256::from(1);
        // let nominator2 = U256::from(2);

        // log::debug!("Setting up network with netuid: {}", netuid);
        // log::debug!("Hotkey: {:?}, Coldkey: {:?}", hotkey, coldkey);
        // log::debug!("Nominators: {:?}, {:?}", nominator1, nominator2);

        // // 2. Create network and register neuron
        // add_network(netuid, 1, 0);
        // register_ok_neuron(netuid, hotkey, coldkey, 100000);
        // SubtensorModule::create_account_if_non_existent(&coldkey, &hotkey);

        // log::debug!("Network created and neuron registered");

        // // 3. Set up balances and stakes
        // SubtensorModule::add_balance_to_coldkey_account(&coldkey, 1000);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator1, 1500);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator2, 1500);

        // log::debug!("Balances added to accounts");

        // // 4. Make the hotkey a delegate
        // let val_take = (u16::MAX as u64 / 10);
        // assert_ok!(SubtensorModule::do_become_delegate(
        //     RuntimeOrigin::signed(coldkey),
        //     hotkey,
        //     val_take as u16
        // ));

        // log::debug!("Hotkey became a delegate with minimum take");

        // // Add stakes for nominators
        // // Add the stake directly to their coldkey-hotkey account
        // // This bypasses the accounting in stake delta
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator1, &hotkey, 100);
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator2, &hotkey, 100);

        // // Do an remove_stake for nominator 1
        // assert_ok!(SubtensorModule::remove_stake(
        //     RuntimeOrigin::signed(nominator1),
        //     hotkey,
        //     12
        // ));
        // // Do an add_stake for nominator 1 of the same amount
        // assert_ok!(SubtensorModule::add_stake(
        //     RuntimeOrigin::signed(nominator1),
        //     hotkey,
        //     12
        // )); // The viable stake should match the initial stake, because the delta is 0

        // // Log the stakes for hotkey, nominator1, and nominator2
        // log::debug!(
        //     "Initial stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}",
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey)
        // );
        // log::debug!("Stakes added for nominators");

        // let nominator1_stake_before =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // // Notice that nominator1 stake is the unchanged from the initial stake
        // assert_eq!(nominator1_stake_before, 100);

        // // 5. Set emission and verify initial states
        // SubtensorModule::set_emission_values(&[netuid], vec![10]).unwrap();
        // assert_eq!(SubtensorModule::get_subnet_emission_value(netuid), 10);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // assert_eq!(SubtensorModule::get_total_stake_for_hotkey(&hotkey), 200);
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);

        // log::debug!("Emission set and initial states verified");

        // // 6. Set hotkey emission tempo
        // SubtensorModule::set_hotkey_emission_tempo(1);
        // log::debug!("Hotkey emission tempo set to 1");

        // // 7. Simulate blocks and check emissions
        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 10);
        // log::debug!(
        //     "After first block, pending emission: {}",
        //     SubtensorModule::get_pending_emission(netuid)
        // );

        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // log::debug!("After second block, pending emission drained");

        // // 8. Check final stakes
        // let delegate_stake = SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey);
        // let total_hotkey_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // let nominator1_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // let nominator2_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey);

        // log::debug!(
        //     "Final stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}, Total Hotkey Stake: {}",
        //     delegate_stake,
        //     nominator1_stake,
        //     nominator2_stake,
        //     total_hotkey_stake
        // );

        // // 9. Verify distribution
        // let min_take = val_take;
        // let total_emission = 20; // 10 per block for 2 blocks
        // let hotkey_emission = total_emission * min_take / u16::MAX as u64;
        // let remaining_emission = total_emission - hotkey_emission;

        // let nominator_1_emission = remaining_emission * nominator1_stake / total_hotkey_stake;
        // let nominator_2_emission = remaining_emission * nominator2_stake / total_hotkey_stake;

        // log::debug!(
        //     "Calculated emissions - Hotkey: {}, Each Nominator: 1;{}, 2;{}",
        //     hotkey_emission,
        //     nominator_1_emission,
        //     nominator_2_emission
        // );

        // // Debug: Print the actual stakes
        // log::debug!("Actual hotkey stake: {}", delegate_stake);
        // log::debug!("Actual nominator1 stake: {}", nominator1_stake);
        // log::debug!("Actual nominator2 stake: {}", nominator2_stake);

        // // Debug: Check the total stake for the hotkey
        // let total_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // log::debug!("Total stake for hotkey: {}", total_stake);

        // // Assertions
        // assert_eq!(delegate_stake, 2, "Hotkey stake mismatch");
        // assert_eq!(
        //     nominator1_stake,
        //     100 + nominator_1_emission, // We expect the emission to be calculated based on the initial stake
        //     // Because the delta is 0.
        //     "Nominator1 stake mismatch"
        // );
        // assert_eq!(
        //     nominator2_stake,
        //     100 + nominator_2_emission,
        //     "Nominator2 stake mismatch"
        // );

        // // 10. Check total stake
        // assert_eq!(total_stake, 200 + total_emission, "Total stake mismatch");

        // log::debug!("Test completed");
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --package pallet-subtensor --test coinbase test_coinbase_nominator_drainage_with_net_positive_delta -- --nocapture
#[test]
fn test_coinbase_nominator_drainage_with_net_positive_delta() {
    new_test_ext(1).execute_with(|| {
        assert!(false);

        // // 1. Set up the network and accounts
        // let netuid: u16 = 1;
        // let hotkey = U256::from(0);
        // let coldkey = U256::from(3);
        // let nominator1 = U256::from(1);
        // let nominator2 = U256::from(2);

        // log::debug!("Setting up network with netuid: {}", netuid);
        // log::debug!("Hotkey: {:?}, Coldkey: {:?}", hotkey, coldkey);
        // log::debug!("Nominators: {:?}, {:?}", nominator1, nominator2);

        // // 2. Create network and register neuron
        // add_network(netuid, 1, 0);
        // register_ok_neuron(netuid, hotkey, coldkey, 100000);
        // SubtensorModule::create_account_if_non_existent(&coldkey, &hotkey);

        // log::debug!("Network created and neuron registered");

        // // 3. Set up balances and stakes
        // SubtensorModule::add_balance_to_coldkey_account(&coldkey, 1000);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator1, 1500);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator2, 1500);

        // log::debug!("Balances added to accounts");

        // // 4. Make the hotkey a delegate
        // let val_take = (u16::MAX as u64 / 10);
        // assert_ok!(SubtensorModule::do_become_delegate(
        //     RuntimeOrigin::signed(coldkey),
        //     hotkey,
        //     val_take as u16
        // ));

        // log::debug!("Hotkey became a delegate with minimum take");

        // // Add stakes for nominators
        // // Add the stake directly to their coldkey-hotkey account
        // // This bypasses the accounting in stake delta
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator1, &hotkey, 100);
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator2, &hotkey, 100);

        // let initial_nominator1_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // let intial_total_hotkey_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // let initial_nominator2_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey);

        // assert_eq!(initial_nominator1_stake, initial_nominator2_stake); // Initial stakes should be equal

        // let removed_stake = 12;
        // // Do an add_stake for nominator 1 of MORE than was removed
        // let added_stake = removed_stake + 1;
        // let net_change: i128 = i128::from(added_stake) - i128::from(removed_stake); // Positive net change

        // // Do an remove_stake for nominator 1
        // assert_ok!(SubtensorModule::remove_stake(
        //     RuntimeOrigin::signed(nominator1),
        //     hotkey,
        //     removed_stake
        // ));

        // // Do an add_stake for nominator 1 of MORE than was removed
        // assert_ok!(SubtensorModule::add_stake(
        //     RuntimeOrigin::signed(nominator1),
        //     hotkey,
        //     added_stake
        // )); // We should expect the emissions to be impacted;
        //     // The viable stake should be the same initial stake for nominator 1
        //     // NOT the new stake amount, because the delta is net positive

        // // Log the stakes for hotkey, nominator1, and nominator2
        // log::debug!(
        //     "Initial stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}",
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey)
        // );
        // log::debug!("Stakes added for nominators");

        // let nominator_1_stake_before =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // // Notice that nominator1 stake is the new stake, including the removed stake
        // assert_eq!(
        //     nominator_1_stake_before,
        //     u64::try_from(100 + net_change).unwrap()
        // );

        // // 5. Set emission and verify initial states
        // SubtensorModule::set_emission_values(&[netuid], vec![10]).unwrap();
        // assert_eq!(SubtensorModule::get_subnet_emission_value(netuid), 10);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // assert_eq!(
        //     SubtensorModule::get_total_stake_for_hotkey(&hotkey),
        //     u64::try_from(200 + net_change).unwrap()
        // );
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);

        // log::debug!("Emission set and initial states verified");

        // // 6. Set hotkey emission tempo
        // SubtensorModule::set_hotkey_emission_tempo(1);
        // log::debug!("Hotkey emission tempo set to 1");

        // // 7. Simulate blocks and check emissions
        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 10);
        // log::debug!(
        //     "After first block, pending emission: {}",
        //     SubtensorModule::get_pending_emission(netuid)
        // );

        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // log::debug!("After second block, pending emission drained");

        // // 8. Check final stakes
        // let delegate_stake = SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey);
        // let total_hotkey_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // let nominator1_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // let nominator2_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey);

        // log::debug!(
        //     "Final stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}, Total Hotkey Stake: {}",
        //     delegate_stake,
        //     nominator1_stake,
        //     nominator2_stake,
        //     total_hotkey_stake
        // );

        // // 9. Verify distribution
        // let min_take = val_take;
        // let total_emission = 20; // 10 per block for 2 blocks
        // let hotkey_emission = total_emission * min_take / u16::MAX as u64;
        // let remaining_emission = total_emission - hotkey_emission;

        // // We expect to distribute using the initial stake for nominator 1; because the delta is net positive
        // // We also use the INITIAL total hotkey stake
        // let nominator_1_emission =
        //     remaining_emission * initial_nominator1_stake / intial_total_hotkey_stake;
        // let nominator_2_emission =
        //     remaining_emission * initial_nominator2_stake / intial_total_hotkey_stake;

        // log::debug!(
        //     "Calculated emissions - Hotkey: {}, Each Nominator: 1;{}, 2;{}",
        //     hotkey_emission,
        //     nominator_1_emission,
        //     nominator_2_emission
        // );

        // // Debug: Print the actual stakes
        // log::debug!("Actual hotkey stake: {}", delegate_stake);
        // log::debug!("Actual nominator1 stake: {}", nominator1_stake);
        // log::debug!("Actual nominator2 stake: {}", nominator2_stake);

        // // Debug: Check the total stake for the hotkey
        // let total_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // log::debug!("Total stake for hotkey: {}", total_stake);

        // // Assertions
        // assert_eq!(delegate_stake, 2, "Hotkey stake mismatch");
        // assert_eq!(
        //     nominator1_stake,
        //     u64::try_from(
        //         net_change
        //             .checked_add_unsigned(100 + nominator_1_emission as u128)
        //             .unwrap()
        //     )
        //     .unwrap(),
        //     "Nominator1 stake mismatch"
        // );
        // assert_eq!(
        //     nominator2_stake,
        //     initial_nominator2_stake + nominator_2_emission,
        //     "Nominator2 stake mismatch"
        // );

        // // 10. Check total stake
        // assert_eq!(
        //     total_stake,
        //     u64::try_from(
        //         net_change
        //             .checked_add_unsigned(200 + total_emission as u128)
        //             .unwrap()
        //     )
        //     .unwrap(),
        //     "Total stake mismatch"
        // );

        // log::debug!("Test completed");
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --package pallet-subtensor --test coinbase test_coinbase_nominator_drainage_with_net_negative_delta -- --nocapture
#[test]
fn test_coinbase_nominator_drainage_with_net_negative_delta() {
    new_test_ext(1).execute_with(|| {
        assert!(false);

        // // 1. Set up the network and accounts
        // let netuid: u16 = 1;
        // let hotkey = U256::from(0);
        // let coldkey = U256::from(3);
        // let nominator1 = U256::from(1);
        // let nominator2 = U256::from(2);

        // log::debug!("Setting up network with netuid: {}", netuid);
        // log::debug!("Hotkey: {:?}, Coldkey: {:?}", hotkey, coldkey);
        // log::debug!("Nominators: {:?}, {:?}", nominator1, nominator2);

        // // 2. Create network and register neuron
        // add_network(netuid, 1, 0);
        // register_ok_neuron(netuid, hotkey, coldkey, 100000);
        // SubtensorModule::create_account_if_non_existent(&coldkey, &hotkey);

        // log::debug!("Network created and neuron registered");

        // // 3. Set up balances and stakes
        // SubtensorModule::add_balance_to_coldkey_account(&coldkey, 1000);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator1, 1500);
        // SubtensorModule::add_balance_to_coldkey_account(&nominator2, 1500);

        // log::debug!("Balances added to accounts");

        // // 4. Make the hotkey a delegate
        // let val_take = (u16::MAX as u64 / 10);
        // assert_ok!(SubtensorModule::do_become_delegate(
        //     RuntimeOrigin::signed(coldkey),
        //     hotkey,
        //     val_take as u16
        // ));

        // log::debug!("Hotkey became a delegate with minimum take");

        // // Add stakes for nominators
        // // Add the stake directly to their coldkey-hotkey account
        // // This bypasses the accounting in stake delta
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator1, &hotkey, 300);
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&nominator2, &hotkey, 300);

        // let initial_nominator1_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // let intial_total_hotkey_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // let initial_nominator2_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey);

        // assert_eq!(initial_nominator1_stake, initial_nominator2_stake); // Initial stakes should be equal

        // let removed_stake = 220;
        // // Do an add_stake for nominator 1 of LESS than was removed
        // let added_stake = removed_stake - 188;
        // let net_change: i128 = i128::from(added_stake) - i128::from(removed_stake); // Negative net change
        // assert!(net_change < 0);

        // // Do an remove_stake for nominator 1
        // assert_ok!(SubtensorModule::remove_stake(
        //     RuntimeOrigin::signed(nominator1),
        //     hotkey,
        //     removed_stake
        // ));

        // // Do an add_stake for nominator 1 of MORE than was removed
        // assert_ok!(SubtensorModule::add_stake(
        //     RuntimeOrigin::signed(nominator1),
        //     hotkey,
        //     added_stake
        // )); // We should expect the emissions to be impacted;
        //     // The viable stake should be the LESS than the initial stake for nominator 1
        //     // Which IS the new stake amount, because the delta is net negative

        // // Log the stakes for hotkey, nominator1, and nominator2
        // log::debug!(
        //     "Initial stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}",
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey),
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey)
        // );
        // log::debug!("Stakes added for nominators");

        // let total_stake_before = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // let nominator_1_stake_before =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // // Notice that nominator1 stake is the new stake, including the removed stake
        // assert_eq!(
        //     nominator_1_stake_before,
        //     u64::try_from(300 + net_change).unwrap()
        // );

        // // 5. Set emission and verify initial states
        // let to_emit = 10_000e9 as u64;
        // SubtensorModule::set_emission_values(&[netuid], vec![to_emit]).unwrap();
        // assert_eq!(SubtensorModule::get_subnet_emission_value(netuid), to_emit);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // assert_eq!(
        //     SubtensorModule::get_total_stake_for_hotkey(&hotkey),
        //     u64::try_from(600 + net_change).unwrap()
        // );
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);

        // log::debug!("Emission set and initial states verified");

        // // 6. Set hotkey emission tempo
        // SubtensorModule::set_hotkey_emission_tempo(1);
        // log::debug!("Hotkey emission tempo set to 1");

        // // 7. Simulate blocks and check emissions
        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), to_emit);
        // log::debug!(
        //     "After first block, pending emission: {}",
        //     SubtensorModule::get_pending_emission(netuid)
        // );

        // next_block();
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);
        // log::debug!("After second block, pending emission drained");

        // // 8. Check final stakes
        // let delegate_stake = SubtensorModule::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey);
        // let total_hotkey_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // let nominator1_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator1, &hotkey);
        // let nominator2_stake =
        //     SubtensorModule::get_stake_for_coldkey_and_hotkey(&nominator2, &hotkey);

        // log::debug!(
        //     "Final stakes - Hotkey: {}, Nominator1: {}, Nominator2: {}, Total Hotkey Stake: {}",
        //     delegate_stake,
        //     nominator1_stake,
        //     nominator2_stake,
        //     total_hotkey_stake
        // );

        // // 9. Verify distribution
        // let min_take = val_take;
        // let total_emission = to_emit * 2; // 10 per block for 2 blocks
        // let hotkey_emission = total_emission * min_take / u16::MAX as u64;
        // let remaining_emission = total_emission - hotkey_emission;

        // // We expect to distribute using the NEW stake for nominator 1; because the delta is net negative
        // // We also use the INITIAL total hotkey stake
        // // Note: nominator_1_stake_before is the new stake for nominator 1, before the epochs run
        // let nominator_1_emission =
        //     remaining_emission * nominator_1_stake_before / total_stake_before;
        // let nominator_2_emission =
        //     remaining_emission * initial_nominator2_stake / total_stake_before;

        // log::debug!(
        //     "Calculated emissions - Hotkey: {}, Each Nominator: 1;{}, 2;{}",
        //     hotkey_emission,
        //     nominator_1_emission,
        //     nominator_2_emission
        // );

        // // Debug: Print the actual stakes
        // log::debug!("Actual hotkey stake: {}", delegate_stake);
        // log::debug!("Actual nominator1 stake: {}", nominator1_stake);
        // log::debug!("Actual nominator2 stake: {}", nominator2_stake);

        // // Debug: Check the total stake for the hotkey
        // let total_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // log::debug!("Total stake for hotkey: {}", total_stake);

        // // Do a fuzzy check on the final stakes
        // let eps = 0.2e9 as u64;

        // let expected_delegate_stake: u64 = 2_000e9 as u64;
        // assert!(
        //     expected_delegate_stake - eps <= delegate_stake
        //         && expected_delegate_stake + eps >= delegate_stake,
        //     "Hotkey stake mismatch - Expected: {}, Actual: {}",
        //     expected_delegate_stake,
        //     delegate_stake
        // );

        // let expected_1_stake = u64::try_from(
        //     net_change
        //         .checked_add_unsigned((initial_nominator1_stake + nominator_1_emission) as u128)
        //         .unwrap(),
        // )
        // .unwrap();
        // assert!(
        //     expected_1_stake - eps <= nominator1_stake
        //         && expected_1_stake + eps >= nominator1_stake,
        //     "Nominator1 stake mismatch - Expected: {}, Actual: {}",
        //     expected_1_stake,
        //     nominator1_stake
        // );
        // let expected_2_stake = initial_nominator2_stake + nominator_2_emission;
        // assert!(
        //     expected_2_stake - eps <= nominator2_stake
        //         && expected_2_stake + eps >= nominator2_stake,
        //     "Nominator2 stake mismatch - Expected: {}, Actual: {}",
        //     expected_2_stake,
        //     nominator2_stake
        // );

        // // 10. Check total stake
        // assert_eq!(
        //     total_stake,
        //     u64::try_from(
        //         net_change
        //             .checked_add_unsigned(
        //                 (initial_nominator2_stake + initial_nominator1_stake + total_emission)
        //                     as u128
        //             )
        //             .unwrap()
        //     )
        //     .unwrap(),
        //     "Total stake mismatch"
        // );

        // log::debug!("Test completed");
    });
}

/// Tests that emission rewards are not distributed when subnet registration is disabled
/// This test verifies that:
/// 1. A subnet with registration disabled does not distribute emissions
/// 2. Pending emissions remain at 0 even after multiple blocks
/// 3. Total stake remains unchanged when registration is disabled
// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --package pallet-subtensor --test coinbase test_emission_with_registration_disabled_subnet -- --nocapture

#[test]
fn test_emission_with_registration_disabled_subnet() {
    new_test_ext(1).execute_with(|| {
        assert!(false);

        // // Initialize test network and accounts
        // let netuid: u16 = 1;
        // let hotkey = U256::from(0); // Validator hotkey
        // let coldkey = U256::from(1); // Validator coldkey

        // // Create network and disable registration
        // add_network(netuid, 1, 0); // Creates subnet with netuid=1, tempo=1, modality=0
        // SubtensorModule::set_network_registration_allowed(netuid, false); // Disable registration

        // // Set up validator accounts and stake
        // // This simulates an existing validator before registration was disabled
        // SubtensorModule::create_account_if_non_existent(&coldkey, &hotkey);
        // SubtensorModule::increase_stake_on_coldkey_hotkey_account(&coldkey, &hotkey, 1000);

        // // Configure emission rate for the subnet
        // SubtensorModule::set_emission_values(&[netuid], vec![10]).unwrap();
        // assert_eq!(SubtensorModule::get_subnet_emission_value(netuid), 10);

        // // Verify initial emission state is zero
        // assert_eq!(SubtensorModule::get_pending_emission(netuid), 0);
        // assert_eq!(SubtensorModule::get_pending_hotkey_emission(&hotkey), 0);

        // // Advance chain by 100 blocks
        // step_block(100);

        // // Verify no emissions were distributed after 100 blocks
        // assert_eq!(
        //     SubtensorModule::get_pending_hotkey_emission(&hotkey),
        //     0,
        //     "Hotkey pending emission should remain zero"
        // );

        // // Advance chain by 1000 more blocks
        // step_block(1000);

        // // Verify total stake remains unchanged after many blocks
        // // This confirms no emissions were added to stake
        // let total_stake = SubtensorModule::get_total_stake_for_hotkey(&hotkey);
        // assert_eq!(
        //     total_stake, 1000,
        //     "Total stake should not increase when registration is disabled"
        // );
    });
}
