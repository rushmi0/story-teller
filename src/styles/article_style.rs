pub const STYLE: &str = r#"

.article-box {
    max-width: 1000px; /* ปรับขนาดความกว้างให้การ์ด */
    margin: 20px auto; /* จัดการ์ดให้อยู่ตรงกลาง */
    padding: 20px;
    border-radius: 10px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    background-color: #404657;
}

.article-field {
    display: flex;
    flex-direction: column;
    gap: 15px;
}

.article-field-text-title {
    color: #FFFFFF;
    font-size: 24px;
    font-weight: bold;
    margin-bottom: 10px;
    text-align: center;
}

.article-field-image-header {
    position: relative;
    width: 100%;
    height: auto;
}

.field-image-box {
    width: 100%;
    height: auto;
    border-radius: 8px;
    object-fit: cover;
}

.field-pt {
    max-width: 600px;
    margin: 0 auto;    /* จัดให้ div อยู่ตรงกลาง */
    padding: 5px;     /* เพิ่ม padding ภายใน */
}

.article-field-icons {
    color: #FFFFFF;
    display: flex;
    justify-content: space-between;
    margin-top: 10px;
}

.field-icon-box {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 14px;
}

.field-icon-box img {
    width: 23px;        /* ปรับขนาดความกว้างของไอคอน */
    height: 23px;       /* ปรับขนาดความสูงของไอคอน */
    object-fit: cover;  /* ให้ขนาดของไอคอนไม่บิดเบี้ยว */
}


.article-field-text {
    font-size: 16px;
    line-height: 1.6;
    color: #FFFFFF;
    margin-top: 10px;
}


"#;