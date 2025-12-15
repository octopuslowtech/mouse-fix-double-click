# Hướng Dẫn Tạo Task Từ PRD

## Ý Tưởng Cốt Lõi

Quy trình này giúp bạn xây dựng các tính năng một cách có cấu trúc và hiệu quả thông qua 3 bước chính:

1. **Xác định phạm vi**: Phác thảo rõ ràng những gì cần được xây dựng bằng Tài liệu Yêu cầu Sản phẩm (PRD).
2. **Lập kế hoạch chi tiết**: Chia nhỏ PRD thành danh sách các nhiệm vụ cụ thể, có thể thực hiện được.
3. **Triển khai lặp đi lặp lại**: Hướng dẫn AI thực hiện từng nhiệm vụ một, cho phép bạn xem xét và phê duyệt từng thay đổi.

Cách tiếp cận có cấu trúc này giúp đảm bảo AI hoạt động đúng hướng, giúp việc gỡ lỗi dễ dàng hơn và mang lại cho bạn sự tự tin vào mã được tạo ra.

## Quy Trình Thực Hiện

### Bước 1: Tạo PRD (Product Requirements Document)

Sử dụng template `@create-prd.md` để tạo tài liệu PRD cho tính năng của bạn:

```
Use @create-prd.md
Here's the feature I want to build: [Mô tả chi tiết tính năng của bạn]
```

### Bước 2: Tạo Danh Sách Task

Sau khi có PRD, sử dụng `@generate-tasks.md` để chuyển đổi PRD thành danh sách task cụ thể:

```
Now take @MyFeature-PRD.md and create tasks using @generate-tasks.md
```

### Bước 3: Bắt Đầu Triển Khai

Khi đã có danh sách task, bắt đầu thực hiện từng task một:

```
Please start on task 1.1 from the generated task list.
```

## Lưu Ý

- Luôn bắt đầu với PRD để đảm bảo phạm vi rõ ràng
- Chia nhỏ task thành các bước có thể thực hiện được
- Xem xét và phê duyệt từng thay đổi trước khi chuyển sang task tiếp theo
- Sử dụng các file template được cung cấp để đảm bảo tính nhất quán
