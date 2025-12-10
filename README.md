# 🕵️ uggo-detecter

> **Tự động nhận diện và hiển thị bảng ngọc, trang bị cho tướng Liên Minh Huyền Thoại.**

![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS-blue)
![Language](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)

**uggo-detecter** là một bản fork nâng cấp của `uggo` CLI. Phiên bản này tập trung vào tính năng **Auto-Detection** (Tự động nhận diện), giúp bạn không cần phải gõ tên tướng thủ công mỗi khi vào trận.

![Demo](assets/usage.gif)

## ✨ Tính năng nổi bật

* 🤖 **Auto-Detect:** Tự động phát hiện tướng bạn chọn trong màn hình Champ Select (Cấm/Chọn).
* ⚡ **Siêu nhẹ:** Viết bằng Rust, chạy ngay trên Terminal, không ngốn RAM như các app Electron (Blitz, OP.GG...).
* 🔄 **Auto-Import:** Tự động đẩy bảng ngọc (Runes) chuẩn từ u.gg vào Client game.
* 📊 **Đa dạng chế độ:** Hỗ trợ Summoner's Rift (5v5), ARAM, và **Arena**.

## 🚀 Cài đặt & Sử dụng

### 1. Tải về
Vào mục [Releases](https://github.com/nguyenkhacvan/uggo-detecter/releases) để tải file chạy (`.exe` cho Windows hoặc binary cho macOS) mới nhất.

### 2. Chạy ứng dụng
1.  Mở **League of Legends Client**.
2.  Chạy file `uggo.exe`.
3.  Vào trận và chọn tướng. Ứng dụng sẽ tự động hiển thị bảng ngọc và lên đồ!

### 3. Phím tắt (Hotkeys)
* `?`: Hiển thị bảng trợ giúp.
* `Alt + s`: Tìm kiếm tướng thủ công (nếu cần).
* `Alt + m`: Đổi chế độ chơi (Normal, ARAM, Arena).
* `Alt + r`: Đổi vị trí (Top, Mid, Jungle...).
* `Ctrl + q`: Thoát ứng dụng.

## 🛠️ Dành cho Developer

Yêu cầu: `Rust 1.89+`

```bash
# Clone dự án
git clone [https://github.com/nguyenkhacvan/uggo-detecter.git](https://github.com/nguyenkhacvan/uggo-detecter.git)
cd uggo-detecter

# Chạy thử (Debug mode)
cargo run

# Build bản release (Tối ưu hóa)
cargo build --release
