use ckb_hash::blake2b_256;
use das_types::{constants::*, out_point, packed::*, prelude::*, util as das_util};
use faster_hex::hex_string;
use util::{gen_price_config, gen_timestamp, prepend_molecule_like_length, read_lines};

mod constants;
mod util;
use hex;
use constants::*;

macro_rules! gen_return_from_entity {
    ( $config_type:expr, $entity:expr ) => {{
        let config_type = ($config_type as u32).to_le_bytes();
        let cell_data = Bytes::from(blake2b_256($entity.as_slice()).to_vec());
        let action_witness = das_util::wrap_action_witness("config", None);

        let cell_witness = das_util::wrap_entity_witness($config_type, $entity);

        // println!(
        //     "size of {:?}: {}",
        //     $config_type,
        //     cell_witness.as_slice().len()
        // );
        if cell_witness.as_slice().len() > WITNESS_SIZE_LIMIT {
            panic!("The size of {:?} is more than {} bytes, this needs to modify das-contracts to support.", $config_type, WITNESS_SIZE_LIMIT)
        }

        format!(
            "0x{} 0x{} 0x{} 0x{}",
            hex_string(&config_type).unwrap(),
            hex_string(cell_data.as_reader().raw_data()).unwrap(),
            hex_string(action_witness.as_reader().raw_data()).unwrap(),
            hex_string(cell_witness.as_reader().raw_data()).unwrap(),
        )
    }};
}

macro_rules! gen_return_from_raw {
    ( $config_type:expr, $entity:expr ) => {{
        let config_type = ($config_type as u32).to_le_bytes();
        let cell_data = Bytes::from(blake2b_256($entity.as_slice()).to_vec());
        let action_witness = das_util::wrap_action_witness("config", None);

        let cell_witness = das_util::wrap_raw_witness($config_type, $entity);

        // println!(
        //     "size of {:?}: {}",
        //     $config_type,
        //     cell_witness.as_slice().len()
        // );
        if cell_witness.as_slice().len() > WITNESS_SIZE_LIMIT {
            panic!("The size of {:?} is more than {} bytes, this needs to modify das-contracts to support.", $config_type, WITNESS_SIZE_LIMIT)
        }

        format!(
            "0x{} 0x{} 0x{} 0x{}",
            hex_string(&config_type).unwrap(),
            hex_string(cell_data.as_reader().raw_data()).unwrap(),
            hex_string(action_witness.as_reader().raw_data()).unwrap(),
            hex_string(cell_witness.as_reader().raw_data()).unwrap(),
        )
    }};
}

fn gen_config_cell_account() -> String {
    let entity = ConfigCellAccount::new_builder()
        .max_length(Uint32::from(42))
        // The basic_capacity contains 1 CKB for kinds of fees
        .basic_capacity(Uint64::from(20_600_000_000))
        .prepared_fee_capacity(Uint64::from(100_000_000))
        .expiration_grace_period(Uint32::from(2_592_000))
        .record_min_ttl(Uint32::from(300))
        .record_size_limit(Uint32::from(5000))
        .transfer_account_fee(Uint64::from(10_000))
        .edit_manager_fee(Uint64::from(10_000))
        .edit_records_fee(Uint64::from(10_000))
        .transfer_account_throttle(Uint32::from(300))
        .edit_manager_throttle(Uint32::from(300))
        .edit_records_throttle(Uint32::from(300))
        .build();

    gen_return_from_entity!(DataType::ConfigCellAccount, entity)
}

fn gen_config_cell_apply() -> String {
    let entity = ConfigCellApply::new_builder()
        .apply_min_waiting_block_number(Uint32::from(1))
        .apply_max_waiting_block_number(Uint32::from(5760))
        .build();

    gen_return_from_entity!(DataType::ConfigCellApply, entity)
}

fn gen_config_cell_income() -> String {
    let entity = ConfigCellIncome::new_builder()
        .basic_capacity(Uint64::from(20_000_000_000))
        .max_records(Uint32::from(50))
        .min_transfer_capacity(Uint64::from(9_000_000_000))
        .build();

    gen_return_from_entity!(DataType::ConfigCellIncome, entity)
}

fn gen_config_cell_main() -> String {
    // ⚠️ Do not modify the following lines of type_id_table,
    // it will be use for search and replace in deploy scripts.
    let type_id_table = TypeIdTable::new_builder()
        .account_cell(Hash::from([]))
        .apply_register_cell(Hash::from([]))
        .balance_cell(Hash::from([]))
        .income_cell(Hash::from([]))
        .pre_account_cell(Hash::from([]))
        .proposal_cell(Hash::from([]))
        .build();

    let das_lock_out_point_table = DasLockOutPointTable::new_builder()
        .ckb_signall(out_point!([], 0))
        // .ckb_multisign(out_point!([], 0))
        // .ckb_anyone_can_pay(out_point!([], 0))
        .eth(out_point!([], 0))
        .tron(out_point!([], 0))
        .build();

    let entity = ConfigCellMain::new_builder()
        .status(Uint8::from(1))
        .type_id_table(type_id_table)
        .das_lock_out_point_table(das_lock_out_point_table)
        .build();

    gen_return_from_entity!(DataType::ConfigCellMain, entity)
}

fn gen_config_cell_price() -> String {
    let discount = DiscountConfig::new_builder()
        .invited_discount(Uint32::from(500))
        .build();

    #[cfg(feature = "mainnet")]
    let prices = PriceConfigList::new_builder()
        .push(gen_price_config(1, 1024_000_000, 1024_000_000))
        .push(gen_price_config(2, 1024_000_000, 1024_000_000))
        .push(gen_price_config(3, 1024_000_000, 1024_000_000))
        .push(gen_price_config(4, 1024_000_000, 1024_000_000))
        .push(gen_price_config(5, 5_000_000, 5_000_000))
        .push(gen_price_config(6, 5_000_000, 5_000_000))
        .push(gen_price_config(7, 5_000_000, 5_000_000))
        .push(gen_price_config(8, 5_000_000, 5_000_000))
        .build();

    #[cfg(not(feature = "mainnet"))]
    let prices = PriceConfigList::new_builder()
        .push(gen_price_config(1, u64::MAX, u64::MAX))
        .push(gen_price_config(2, 30_000_000, 30_000_000))
        .push(gen_price_config(3, 20_000_000, 20_000_000))
        .push(gen_price_config(4, 10_000_000, 10_000_000))
        .push(gen_price_config(5, 5_000_000, 5_000_000))
        .push(gen_price_config(6, 5_000_000, 5_000_000))
        .push(gen_price_config(7, 5_000_000, 5_000_000))
        .push(gen_price_config(8, 5_000_000, 5_000_000))
        .build();

    let entity = ConfigCellPrice::new_builder()
        .discount(discount)
        .prices(prices)
        .build();

    gen_return_from_entity!(DataType::ConfigCellPrice, entity)
}

fn gen_config_cell_proposal() -> String {
    let entity = ConfigCellProposal::new_builder()
        .proposal_min_confirm_interval(Uint8::from(2))
        .proposal_min_extend_interval(Uint8::from(1))
        .proposal_min_recycle_interval(Uint8::from(8))
        .proposal_max_account_affect(Uint32::from(50))
        .proposal_max_pre_account_contain(Uint32::from(50))
        .build();

    gen_return_from_entity!(DataType::ConfigCellProposal, entity)
}

fn gen_config_cell_profit_rate() -> String {
    let entity = ConfigCellProfitRate::new_builder()
        .channel(Uint32::from(1000))
        .inviter(Uint32::from(1000))
        .proposal_create(Uint32::from(200))
        .proposal_confirm(Uint32::from(0))
        .income_consolidate(Uint32::from(500))
        .build();

    gen_return_from_entity!(DataType::ConfigCellProfitRate, entity)
}

fn gen_config_cell_record_key_namespace() -> String {
    let mut record_key_namespace = Vec::new();
    let lines = read_lines("record_key_namespace.txt")
        .expect("Expect file ./data/record_key_namespace.txt exist.");
    for line in lines {
        if let Ok(key) = line {
            record_key_namespace.push(key);
        }
    }
    record_key_namespace.sort();

    // Join all record keys with 0x00 byte as entity.
    let mut raw = Vec::new();
    for key in record_key_namespace {
        raw.extend(key.as_bytes());
        raw.extend(&[0u8]);
    }
    let raw = prepend_molecule_like_length(raw);

    gen_return_from_raw!(DataType::ConfigCellRecordKeyNamespace, raw)
}

fn gen_config_cell_preserved_account() -> String {
    // Load and group preserved accounts
    let mut preserved_accounts_groups: Vec<Vec<Vec<u8>>> =
        vec![Vec::new(); PRESERVED_ACCOUNT_CELL_COUNT as usize];
    let lines = read_lines("preserved_accounts.txt")
        .expect("Expect file ./data/preserved_accounts.txt exist.");
    for line in lines {
        if let Ok(account) = line {
            let account_hash = blake2b_256(account.as_bytes())
                .get(..ACCOUNT_ID_LENGTH)
                .unwrap()
                .to_vec();
            let index = (account_hash[0] % PRESERVED_ACCOUNT_CELL_COUNT) as usize;

            preserved_accounts_groups[index].push(account_hash);
        }
    }

    let mut output = String::new();
    let mut comma = "";
    for (_i, mut group) in preserved_accounts_groups.into_iter().enumerate() {
        // println!("Preserved account group[{}] count: {}", _i, group.len());
        if group.len() > PRESERVED_ACCOUNT_LIMIT_PER_CELL {
            panic!("Some ConfigCell of preserved accounts has broke the predict limitation.")
        }

        group.sort();
        let mut raw = group.into_iter().flatten().collect::<Vec<u8>>();
        raw = prepend_molecule_like_length(raw);

        let data_type = das_util::preserved_accounts_group_to_data_type(_i);
        output += comma;
        output += gen_return_from_raw!(data_type, raw).as_str();
        comma = ",";
    }

    output
}

fn gen_config_cell_char_set() -> String {
    let settings: Vec<(DataType, &str, u8)> = vec![
        (DataType::ConfigCellCharSetEmoji, "char_set_emoji.txt", 1),
        (DataType::ConfigCellCharSetDigit, "char_set_digit.txt", 1),
        (DataType::ConfigCellCharSetEn, "char_set_en.txt", 0),
        // (DataType::ConfigCellCharSetZhHans, "char_set_zh_hans.txt", 0),
        // (DataType::ConfigCellCharSetZhHant, "char_set_zh_hant.txt", 0),
    ];

    let mut output = String::new();
    let mut comma = "";
    for (_i, setting) in settings.iter().enumerate() {
        let mut charsets = Vec::new();
        let lines = read_lines(setting.1)
            .expect(format!("Expect file ./data/{} exist.", setting.1).as_str());
        for line in lines {
            if let Ok(char) = line {
                charsets.push(char);
            }
        }

        // println!("Character count of {:?}: {}", setting.0, charsets.len());

        // Join all record keys with 0x00 byte as entity.
        let mut raw: Vec<u8> = Vec::new();
        raw.push(setting.2); // global status
        for key in charsets {
            raw.extend(key.as_bytes());
            raw.extend(&[0u8]);
        }
        let raw = prepend_molecule_like_length(raw);

        output += comma;
        output += gen_return_from_raw!(setting.0, raw).as_str();
        comma = ",";
    }

    output
}

fn gen_config_cell_release() -> String {
    #[cfg(feature = "mainnet")]
    let data = vec![(
        0,
        gen_timestamp("2021-07-01 00:00:00"),
        gen_timestamp("2021-07-01 00:00:00"),
    )];

    #[cfg(not(feature = "mainnet"))]
    let data = vec![
        (
            2,
            gen_timestamp("2021-07-01 00:00:00"),
            gen_timestamp("2021-07-31 00:00:00"),
        ),
        (
            0,
            gen_timestamp("2021-06-1 00:00:00"),
            gen_timestamp("2021-06-1 00:00:00"),
        ),
    ];

    let mut release_rules = ReleaseRules::new_builder();
    for item in data.into_iter() {
        release_rules = release_rules.push(
            ReleaseRule::new_builder()
                .length(Uint32::from(item.0))
                .release_start(Timestamp::from(item.1))
                .release_end(Timestamp::from(item.2))
                .build(),
        );
    }

    let entity = ConfigCellRelease::new_builder()
        .release_rules(release_rules.build())
        .build();

    gen_return_from_entity!(DataType::ConfigCellRelease, entity)
}

// fn gen_config_cell_secondary_market() -> String {
//     let entity = ConfigCellSecondaryMarket::new_builder()
//         .min_sale_price(Uint64::from(20_000_000_000))
//         .sale_expiration_limit(Uint64::from(86400 * 30))
//         .sale_description_bytes_limit(Uint32::from(5000))
//         .build();
//
//     gen_return_from_entity!(DataType::ConfigCellSecondaryMarket, entity)
// }

// fn calc_config_cells_need_update() {
//     use std::collections::HashSet;
//
//     // Load and group preserved accounts
//     let lines =
//         read_lines("new_to_update.txt").expect("Expect file ./data/new_to_updated.txt exist.");
//
//     let mut id_set = HashSet::new();
//
//     for line in lines {
//         if let Ok(account) = line {
//             let account_hash = blake2b_256(account.as_bytes())
//                 .get(..ACCOUNT_ID_LENGTH)
//                 .unwrap()
//                 .to_vec();
//             let index = (account_hash[0] % PRESERVED_ACCOUNT_CELL_COUNT) as usize;
//             let key = hex_string(((10000 + index) as u32).to_le_bytes().as_ref()).unwrap();
//             println!("Because {} need to update 0x{}", account, key);
//
//             id_set.insert(key);
//         }
//     }
//
//     let mut id_vec = id_set.into_iter().collect::<Vec<_>>();
//     id_vec.sort();
//
//     println!();
//     println!("All ConfigCells which need to be updated:");
//     println!();
//     for key in id_vec {
//         println!("0x{}", key)
//     }
// }

/**
this function is nearly the same as the function in template_generator.rs under das-contracts repo.
**/
fn gen_config_cell_unavailable_account() -> String {
    let mut unavailable_account_hashes = Vec::new();
    let lines = util::read_lines("unavailable_account_hashes.txt")
        .expect("Expect file ./data/unavailable_account_hashes.txt exist.");

    for line in lines {
        if let Ok(account_hash_string) = line {
            let account_hash: Vec<u8> = hex::decode(account_hash_string).unwrap();
            unavailable_account_hashes.push(account_hash.get(..ACCOUNT_ID_LENGTH).unwrap().to_vec());
        }
    }

    unavailable_account_hashes.sort(); // todo: maybe we don't need to sort, traverse is just enough

    let mut raw = Vec::new();

    for account_hash in unavailable_account_hashes {
        raw.extend(account_hash);
    }
    let raw = util::prepend_molecule_like_length(raw);

    gen_return_from_raw!(DataType::ConfigCellUnAvailableAccount, raw)
}

fn main() {
    print!("{},", gen_config_cell_account());
    print!("{},", gen_config_cell_apply());
    print!("{},", gen_config_cell_income());
    print!("{},", gen_config_cell_main());
    print!("{},", gen_config_cell_price());
    print!("{},", gen_config_cell_proposal());
    print!("{},", gen_config_cell_profit_rate());
    print!("{},", gen_config_cell_record_key_namespace());
    print!("{},", gen_config_cell_release());
    // print!("{},", gen_config_cell_secondary_market());
    print!("{},", gen_config_cell_preserved_account());
    print!("{},", gen_config_cell_unavailable_account());
    print!("{}", gen_config_cell_char_set());
    print!("\n");

    // calc_config_cells_need_update();
}
