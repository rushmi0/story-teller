pub const STYLE: &str = r#"

.story-write-box {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    border-radius: 8px;
}

.story-header-box {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 5px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    background-color: #404657
}

.title-box {
    display: flex;
    justify-content: space-between;
}


.input-title,
.input-tags {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.input-title span,
.input-tags span {
    font-size: 1rem;
    font-weight: 500;
    color: #fff;
}

.input-tags img {
    width: 20px;
    height: 20px;
    cursor: pointer;
}

.write-box {
    padding: 17px;
    margin-bottom: 10px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    border-radius: 7px;
    background-color: #404657;
}

.write-box h2 {
    font-size: 0.9rem;
    color: #fff;
    margin-bottom: 0.5rem;
}

.option-btn {
    display: flex;
    gap: 0.5rem;
}

.option-btn-item {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    background: #3d424f;
    border: none;
    border-radius: 5px;
    padding: 0.3rem 0.5rem;
    cursor: pointer;
    font-size: 0.9rem;
    color: #fff;
    transition: background 0.3s ease;
}

.option-btn-item:hover {
    background: #3d424f;
    color: #e0e0e0;
}

.option-btn-item img {
    width: 16px;
    height: 16px;
}

.input-long-text {
    min-height: 200px;
    padding: 0.5rem;
    border-radius: 5px;
    background: #333947;
    font-size: 1.1rem;
    color: #fff;
    resize: none;
}

/* จัดให้ submit-btn และ cancel-btn เรียงต่อกัน */
.action-btn {
    display: flex;
    gap: 0.5rem;
}

.submit-btn {
    background-color: #04b85d; /* สีเขียวพาสเทล */
    color: white;
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    transition: background 0.3s ease;
}

.submit-btn:hover {
    background-color: #00FF7F;
}

.cancel-btn {
    background-color: #ef5350; /* สีแดงพาสเทล */
    color: white;
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    transition: background 0.3s ease;
}

.cancel-btn:hover {
    background-color: #e53935;
}




/*********************************/

.markdown-field-text {
    white-space: pre-wrap;
    font-size: 19px;
    line-height: 1.5;
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
    color: #90EE90;
    text-decoration: underline;
    transition: color 0.3s;
    white-space: nowrap;       /* ทำให้ข้อความไม่ขึ้นบรรทัดใหม่ */
    overflow: hidden;          /* ซ่อนข้อความที่เกินขอบเขต */
    text-overflow: ellipsis;   /* แสดงจุดสามจุดเมื่อข้อความเกินขอบเขต */
    max-width: 100%;           /* กำหนดขอบเขตความกว้างสูงสุด */
}

.markdown-field-text a:hover {
    color: #32CD32;
}

.markdown-field-text img {
    margin-top: 10px;
    border-radius: 8px;
    max-width: 100%; /* ให้รูปภาพไม่กว้างเกินขอบเขตของ container */
    height: auto;    /* รักษาอัตราส่วนภาพ */
    object-fit: cover;
}



/* เพิ่มสไตล์สำหรับการแสดงผลตาราง */
.markdown-field-text table {
    width: 100%;
    border-collapse: collapse;
    margin: 20px 0;
    overflow-x: auto;
    display: block;
    white-space: nowrap;
}

/* ปรับให้เซลล์ของตารางเรียงอยู่ตรงกลางและเพิ่มพื้นที่ padding */
.markdown-field-text th,
.markdown-field-text td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: center;
}

/* สไตล์สำหรับ header ของตาราง */
.markdown-field-text th {
    background-color: #404657;
    color: white;
    font-weight: bold;
}

"#;
