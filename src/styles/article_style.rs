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

.markdown-field-text-title {
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

/*******************************/


.markdown-field-text {
    white-space: pre-wrap;
    font-size: 16px;
    line-height: 1.3;
    color: #FFFFFF;
    margin-top: 10px;
}

.markdown-field-text pre {
    background-color: #242430;
    padding: 10px;
    border-radius: 5px;
    overflow-x: auto; /* ทำให้สามารถเลื่อนด้านข้างได้เมื่อเนื้อหายาว */
}

/* สไตล์สำหรับลิงก์ */
.markdown-field-text a {
    color: blue;
    text-decoration: underline;
}

.markdown-field-text a:hover {
    color: darkblue;
}

/* เพิ่มสไตล์สำหรับการแสดงผลตาราง */
.markdown-field-text table {
    width: 100%;
    border-collapse: collapse;
    margin: 20px 0;
}

.markdown-field-text th, .markdown-field-text td {
    border: 1px solid #ddd;
    padding: 8px;
}

.markdown-field-text th {
    background-color: #404657;
    color: white;
    text-align: center;
}

"#;