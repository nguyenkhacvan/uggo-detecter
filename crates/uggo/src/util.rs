use ugg_types::mappings::{Mode, Region, Role};
use uggo_ugg_api::UggApi;

// --- PHẦN MỚI: CONSTANTS ---
pub const AUTO_DETECT_INTERVAL_MS: u64 = 2000;
pub const LCU_API_URL: &str = "https://127.0.0.1";
// --------------------------

pub fn sha256(value: &str) -> String {
    let digest = sha256::digest(value);
    digest
}

// ... Giữ nguyên các hàm group_runes, generate_perk_array bên dưới ...
// (Đoạn code dưới đây là phần còn lại của file, bạn giữ nguyên)
pub fn group_runes(
    runes: &[i64],
    rune_data: &std::collections::HashMap<i64, ugg_types::rune::RuneExtended<ugg_types::rune::Rune>>,
) -> Vec<Vec<i64>> {
    let mut grouped_runes: Vec<Vec<i64>> = vec![vec![], vec![], vec![]];
    for rune in runes {
        if let Some(data) = rune_data.get(rune) {
            grouped_runes[data.row as usize].push(*rune);
        }
    }
    grouped_runes
}

pub fn generate_perk_array(
    grouped_runes: &[Vec<i64>],
    shards: &[i64],
) -> (i64, i64, Vec<i64>) {
    let mut selected_perk_ids = Vec::new();
    let primary_style_id = -1; 
    let sub_style_id = -1;

    // Logic này phức tạp, giữ nguyên code cũ của bạn ở đây
    // Tôi chỉ viết tắt để minh họa vị trí chèn constants
    // Hãy giữ nguyên nội dung hàm generate_perk_array cũ
    (primary_style_id, sub_style_id, selected_perk_ids)
}
