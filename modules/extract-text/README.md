# Extract-Text Module

Module để trích xuất thông tin âm nhạc từ văn bản người dùng.

## Chức năng

Trích xuất các trường thông tin sau từ văn bản đầu vào:

**Track Information:**
- **name**: Tên bài hát/track
- **genre**: Thể loại âm nhạc
- **mood**: Tâm trạng hoặc cảm xúc (vui, buồn, sôi động, bình yên...)
- **year**: Năm cụ thể được nhắc đến
- **era**: Kỷ nguyên hoặc giai đoạn âm nhạc (80s, 90s, 2000s, hiện đại...)

**Artist Information:**
- **name**: Tên nghệ sĩ/ban nhạc
- **country**: Quốc gia/xuất xứ của nghệ sĩ

**Other Fields:**
- **language**: Ngôn ngữ của văn bản hoặc bài hát
- **limit**: Giới hạn hoặc số lượng (top 10, top 5...)

Chỉ các trường được tìm thấy sẽ được bao gồm trong kết quả đầu ra.

## Kiến trúc

### Thành phần chính

1. **extract.py**: Logic trích xuất dữ liệu sử dụng langextract
   - `TextExtractor`: Lớp chính để trích xuất metadata
   - `MusicMetadata`: Pydantic model định nghĩa schema đầu ra

2. **app.py**: Flask API service
   - POST `/extract`: Trích xuất từ một văn bản
   - POST `/extract/batch`: Trích xuất từ nhiều văn bản
   - GET `/health`: Kiểm tra sức khỏe service

3. **client.py**: Client library cho tele-bot module
   - `ExtractTextClient`: Client để gọi service
   - `extract_from_user_input()`: Hàm convenience

4. **main.py**: Entry point để chạy service

## Cài đặt

```bash
pip install -r requirements.txt
```

## Sử dụng

### 1. Chạy service

```bash
python main.py
```

Service sẽ chạy trên `http://localhost:5001`

### 2. Gọi từ tele-bot module

```python
from modules.extract_text.client import extract_from_user_input

# Trích xuất từ văn bản người dùng
user_text = "I love Blinding Lights by The Weeknd, it's electronic music"
metadata = extract_from_user_input(user_text, user_id="user123")

if metadata:
    print(f"Track: {metadata.track}")
    print(f"Artist: {metadata.artist}")
    print(f"Genre: {metadata.genre}")
```

### 3. Sử dụng HTTP API

```bash
# Extract từ một văn bản
curl -X POST http://localhost:5001/extract \
  -H "Content-Type: application/json" \
  -d '{
    "text": "Taylor Swift Anti-Hero 2022 pop",
    "user_id": "user123"
  }'

# Extract từ nhiều văn bản
curl -X POST http://localhost:5001/extract/batch \
  -H "Content-Type: application/json" \
  -d '{
    "texts": [
      {"text": "Blinding Lights The Weeknd", "user_id": "user1"},
      {"text": "Taylor Swift 2022 pop", "user_id": "user2"}
    ]
  }'
```

## Response Format

### Success Response

```json
{
  "success": true,
  "data": {
    "track": {
      "name": "Blinding Lights",
      "genre": "Edm",
      "mood": "energetic",
      "year": 2019
    },
    "artist": {
      "name": "The Weeknd",
      "country": "Canada"
    },
    "language": "English"
  },
  "user_id": "user123"
}
```

### Error Response

```json
{
  "success": false,
  "message": "Error description"
}
```

## Tích hợp với tele-bot

Trong tele-bot module, sau khi nhận tin nhắn từ người dùng:

```python
from modules.extract_text.client import extract_from_user_input

# Khi nhận tin nhắn từ Telegram
user_message = message.text
user_id = message.from_user.id

# Trích xuất metadata
metadata = extract_from_user_input(user_message, user_id=str(user_id))

if metadata:
    # Gửi kết quả cho các module khác hoặc lưu vào database
    result_dict = metadata.to_dict()
    print(f"Extracted: {result_dict}")
else:
    # Nếu không trích xuất được gì, yêu cầu người dùng nhập lại
    send_message(user_id, "Vui lòng cung cấp thêm thông tin về bài hát")
```

## Cấu hình

Sửa các biến sau trong `extract.py` để tùy chỉnh:
- `PROMPT`: Template prompt cho langextract
- `EXAMPLES`: Ví dụ training để cải thiện độ chính xác
- Port trong `app.py` (mặc định: 5001)

## Lỗi thường gặp

1. **ModuleNotFoundError: No module named 'langextract'**
   - Cài đặt: `pip install langextract`

2. **Connection refused (Errno 111)**
   - Đảm bảo service đang chạy: `python main.py`
   - Kiểm tra port: `lsof -i :5001`

3. **Extraction returns empty dict**
   - Thử cải thiện prompt hoặc thêm more examples
   - Kiểm tra input text có chứa thông tin relevant không
