use std::collections::HashMap;
use ugg_types::rune::RuneExtended;

pub const AUTO_DETECT_INTERVAL_MS: u64 = 2000; 

// Hàm nhóm các ngọc theo bảng (VD: Chuẩn xác, Áp đảo...)
// Trả về: (Tên bảng, Danh sách các ngọc trong bảng kèm ID)
pub fn group_runes<'a, T: Clone>(
    rune_ids: &[i64],
    rune_data: &'a HashMap<i64, RuneExtended<T>>,
) -> Vec<(String, Vec<(i64, &'a RuneExtended<T>)>)> {
    let mut grouped_runes: Vec<(String, Vec<(i64, &RuneExtended<T>)>)> = Vec::new();

    for id in rune_ids {
        if let Some(rune) = rune_data.get(id) {
            // Tìm xem bảng ngọc này đã có trong danh sách chưa
            if let Some(pos) = grouped_runes.iter().position(|(name, _)| name == &rune.parent) {
                grouped_runes[pos].1.push((*id, rune));
            } else {
                // Nếu chưa, tạo mới
                grouped_runes.push((rune.parent.clone(), vec![(*id, rune)]));
            }
        }
    }
    
    // Sắp xếp: Bảng ngọc chính (nhiều ngọc hơn) lên trước, bảng phụ ra sau
    grouped_runes.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    grouped_runes
}

// Hàm tạo mảng perk ID để gửi lên Client LMHT
pub fn generate_perk_array<T: Clone>(
    grouped_runes: &[(String, Vec<(i64, &RuneExtended<T>)>)],
    shards: &[i64],
) -> (i64, i64, Vec<i64>) {
    let mut primary_style_id = -1;
    let mut sub_style_id = -1;
    let mut selected_perk_ids = Vec::new();

    for (_, runes) in grouped_runes {
        if let Some((_, first_rune)) = runes.first() {
            // Logic đơn giản: Bảng nào >= 3 ngọc là bảng chính (Primary), còn lại là phụ (Secondary)
            if runes.len() >= 3 { 
                primary_style_id = first_rune.parent_id;
            } else {
                sub_style_id = first_rune.parent_id;
            }
            // Lấy danh sách ID ngọc
            selected_perk_ids.extend(runes.iter().map(|(id, _)| *id));
        }
    }

    // Thêm các mảnh chỉ số (shards)
    selected_perk_ids.extend(shards);

    (primary_style_id, sub_style_id, selected_perk_ids)
}
