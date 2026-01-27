# 🎵 Spotify Dashboard Telegram Bot

Truy cập Spotify của bạn trực tiếp từ Telegram. Xem top tracks, nghệ sĩ yêu thích, bài hát vừa nghe, tạo và quản lý playlist - tất cả trong một bot!

## ✨ Tính Năng

- 🔐 **Xác thực Spotify** - Đăng nhập an toàn với OAuth2
- 🎵 **Top Tracks** - Xem 10 bài hát yêu thích nhất của bạn
- 🎤 **Top Artists** - Xem 10 nghệ sĩ yêu thích nhất
- ⏱️ **Recently Played** - Xem 10 bài hát vừa nghe gần đây
- 👤 **Profile** - Xem thông tin tài khoản Spotify
- 🔍 **Search** - Tìm bài hát trong thư viện của bạn
- 📋 **Playlist** - Tạo, xem và quản lý playlist
- ➕ **Add to Playlist** - Thêm bài hát vào playlist

## 🚀 Cài Đặt

### Yêu Cầu

- Rust 1.70+ (để build)
- Telegram bot token (từ [@BotFather](https://t.me/botfather))
- Spotify API credentials (từ [Spotify Developer Dashboard](https://developer.spotify.com/dashboard))

### Hướng Dẫn

1. **Clone repo**
   ```bash
   git clone <repo-url>
   cd spotify-dashboard
   ```

2. **Cấu hình biến môi trường**
   ```bash
   cp .env.example .env
   ```
   
   Điền vào `.env`:
   - `TELOXIDE_TOKEN` - Token từ @BotFather
   - `RSPOTIFY_CLIENT_ID` - Từ Spotify Dashboard
   - `RSPOTIFY_CLIENT_SECRET` - Từ Spotify Dashboard
   - `RSPOTIFY_REDIRECT_URI` - OAuth callback (ví dụ: http://localhost:3000/callback)

3. **Build và chạy**
   ```bash
   cargo build --release
   ./target/release/spotify-dashboard
   ```

## 📲 Lệnh Bot

| Lệnh | Chức Năng |
|------|-----------|
| `/help` | Hiển thị tất cả lệnh |
| `/login` | Đăng nhập Spotify |
| `/me` | Xem thông tin profile |
| `/top_tracks` | Top 10 bài hát |
| `/top_artists` | Top 10 nghệ sĩ |
| `/recently_played` | 10 bài hát vừa nghe |
| `/search query` | Tìm bài hát |
| `/playlists` | Danh sách playlist |
| `/playlist name` | Chi tiết playlist |
| `/create_playlist name` | Tạo playlist mới |
| `/add_to_playlist song \| playlist` | Thêm bài hát vào playlist |

## 💡 Ví Dụ Sử Dụng

```
/login
👉 Ấn nút để đăng nhập với Spotify

/search imagine
🔍 Kết quả tìm kiếm cho "imagine"
1. Imagine - John Lennon
...

/create_playlist My Favorites
✅ Playlist Created: My Favorites

/add_to_playlist Imagine | My Favorites
✅ Track Added: Imagine → My Favorites
```

## ⚙️ Cấu Hình

Bot tự động lưu session của mỗi user, không cần cấu hình thêm. Chỉ cần set biến môi trường và chạy!

## 📝 License

MIT
