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
- 🎼 **Genre Detection** - Phát hiện thể loại nhạc tự động (Ballad, Pop, Rock, EDM, Hip-Hop, v.v.)

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

## 🎼 Phát Hiện Thể Loại Nhạc (Genre Detection)

Bot sử dụng hệ thống phát hiện thể loại dựa trên quy tắc (rule-based), không dùng AI/LLM. Hệ thống phân tích:

**Audio Features từ Spotify:**
- Tempo, Energy, Valence, Danceability
- Acousticness, Instrumentalness, Loudness, Speechiness

**Metadata của Nghệ Sĩ:**
- Genre tags từ tài khoản Spotify
- Độ nổi tiếng (Popularity)

**Hỗ Trợ 13 Thể Loại:**
- 🎭 **Ballad** - Nhạc chậm, cảm động
- 🎶 **Pop** - Nhạc nhẹ nhàng, vui vẻ
- 🎸 **Rock** - Năng lượng cao, guitar mạnh
- 🎛️ **EDM** - Electronic, bass mạnh, nhảy
- 🎤 **Hip-Hop** - Rap, flow, beat nặng
- 💿 **R&B** - Soul, mượt mà, lãng mạn
- 🎷 **Jazz** - Nhạc jazz, im lặng
- 🎹 **Classical** - Nhạc cổ điển, giao hưởng
- 🎸 **Acoustic** - Acoustic guitar, tự nhiên
- 🌙 **Lo-Fi** - Chillhop, study music
- 🌟 **Indie** - Independent, alternative
- 🤘 **Metal** - Heavy metal, rock nặng
- ❓ **Unknown** - Không xác định

**Ưu Điểm:**
- ✅ Không dùng LLM - Nhanh, tin cậy, không cần internet thêm
- ✅ Hoàn toàn trong suốt - Bạn biết chính xác tại sao một bài hát được phân loại như vậy
- ✅ Hỗ trợ lọc - Dễ tìm bài hát theo thể loại
- ✅ Có thể điều chỉnh - Các quy tắc dễ tuning nếu cần

## ⚙️ Cấu Hình

Bot tự động lưu session của mỗi user, không cần cấu hình thêm. Chỉ cần set biến môi trường và chạy!

## 📝 License

MIT
