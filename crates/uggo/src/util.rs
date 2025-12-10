// File: crates/uggo/src/util.rs

use std::collections::HashMap;
use ugg_types::rune::RuneExtended;

// --- THÊM DÒNG NÀY ĐỂ SỬA LỖI BUILD ---
pub const AUTO_DETECT_INTERVAL_MS: u64 = 2000; 
// --------------------------------------

pub fn group_runes(
    rune_ids: &[i64],
    rune_data: &HashMap<i64, RuneExtended>,
) -> Vec<(i64, Vec<(i64, String)>)> {
    let mut grouped_runes: Vec<(i64, Vec<(i64, String)>)> = Vec::new();

    for rune in rune_ids {
        let rune_info = rune_data.get(rune);
        match rune_info {
            Some(info) => {
                let group_index = grouped_runes
                    .iter()
                    .position(|(id, _)| id == &info.parent_id);
                match group_index {
                    Some(i) => grouped_runes[i].1.push((*rune, info.rune.name.clone())),
                    None => grouped_runes.push((
                        info.parent_id,
                        vec![(*rune, info.rune.name.clone())],
                    )),
                }
            }
            None => continue,
        }
    }

    grouped_runes
}

pub fn generate_perk_array(
    grouped_runes: &[(i64, Vec<(i64, String)>)],
    shards: &[i64],
) -> (i64, i64, Vec<i64>) {
    let mut primary_style_id = -1;
    let mut sub_style_id = -1;
    let mut selected_perk_ids = Vec::new();

    for (style_id, runes) in grouped_runes {
        if runes.len() == 4 {
            primary_style_id = *style_id;
            selected_perk_ids.extend(runes.iter().map(|(id, _)| id));
        } else {
            sub_style_id = *style_id;
            selected_perk_ids.extend(runes.iter().map(|(id, _)| id));
        }
    }

    selected_perk_ids.extend(shards);

    (primary_style_id, sub_style_id, selected_perk_ids)
}
