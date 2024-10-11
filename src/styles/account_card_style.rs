pub const STYLE: &str = r#"

.account-card {
    background-color: #1D212E; /* กำหนดสีพื้นหลัง */
    background-image: radial-gradient(
        ellipse farthest-corner at 45px 45px,
        rgba(64, 70, 87, 0.8) 0%,
        rgba(64, 70, 87, 0.2)
    );
    border-radius: 25px;
    color: #343e66;
    padding-top: 30px;
    position: fixed; /* แก้จาก relative เป็น fixed เพื่อให้ลอยอยู่ */
    top: 60px;  /* ระบุตำแหน่งการ์ดจากด้านบน */
    right: 20px; /* ระบุตำแหน่งการ์ดจากด้านขวา */
    width: 350px;
    max-width: 100%;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center; /* จัดให้อยู่ตรงกลาง */
    z-index: 1000; /* ใช้เพื่อให้การ์ดอยู่บนสุด */
}

.profile-image {
    border: 1px solid #03BFCB;
    border-radius: 50%;
    padding: 7px;
    width: 120px;
    height: 120px;
    object-fit: cover;
    margin-bottom: 20px;
}

.user-info {
    margin-bottom: 20px;
}

.user-name {
    margin: 10px 0;
    font-size: 24px;
    font-weight: bold;
}

.user-location {
    margin: 5px 0;
    text-transform: uppercase;
    font-size: 12px;
    color: #666;
}

.user-description {
    font-size: 14px;
    line-height: 21px;
    color: #444;
}

"#;