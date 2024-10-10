pub const STYLE: &str = r#"

.note-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
}

.note-box {
    background-color: #404657; /* กำหนดสีพื้นหลัง */
    background-image: radial-gradient(
        ellipse farthest-corner at 45px 45px,
        rgba(64, 70, 87, 0.8) 0%,
        rgba(64, 70, 87, 0.2)
    );
    border-radius: 15px;
    transition: 0.3s;
    box-sizing: border-box;
    overflow: hidden;
    width: 100%; /* ปรับให้การ์ดใช้พื้นที่ให้เต็ม */
    max-width: 252px; /* กำหนดขนาดสูงสุด */
    margin: 20px;
}

.note-box:hover {
    border-color: #6c00f8;
    box-shadow: 0 0 12px 4px rgba(243, 236, 248, 0.8);
}

.note-out:hover {
    transform: scale(1.07);
    border: 3px solid #6c00f8;
}



.note-image {
    width: 100%; /* ปรับให้ภาพใช้พื้นที่เต็มการ์ด */
    height: 256px;
    object-fit: cover; /* ให้ภาพไม่บิดเบี้ยว */
    border-radius: 15px 15px 0px 0px;
}

.note-text {
    padding-bottom: 5px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.note-desc {
    padding-top: 12px;
    padding-left: 10px;
    padding-right: 10px;
    font-size: 16px;
    color: #a7aab5;
    line-height: 16px;
}

.note-desc h2 {
    color: #f7f7f7;
    font-size: 18px;
}

.note-icon {
    padding-left: 5px;
    padding-right: 5px;
    padding-top: 10px;
    padding-bottom: 10px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

hr {
    margin-top: 5px;
    border-width: 0.01em;
    border-color: #a7aab5;
    font-size: 1px;
}


@media only screen and (max-width: 923px) {

    .note-box {
        width: 100%; /* ปรับให้การ์ดใช้พื้นที่ให้เต็ม */
        max-width: 185px; /* กำหนดขนาดสูงสุด */
        margin: 20px;
    }

    .note-image {
        width: 100%; /* ปรับให้ภาพใช้พื้นที่เต็มการ์ด */
        height: 189px;
    }

    .note-desc {
        font-size: 14px;
    }

    .note-desc h2 {
        font-size: 16px;
    }


}


@media only screen and (max-width: 721px) {

    .note-box {
        width: 100%; /* ปรับให้การ์ดใช้พื้นที่ให้เต็ม */
        max-width: 156px; /* กำหนดขนาดสูงสุด */
        margin: 20px;
    }

    .note-image {
        width: 100%; /* ปรับให้ภาพใช้พื้นที่เต็มการ์ด */
        height: 170px;
    }

    .note-desc {
        font-size: 12px;
    }

    .note-desc h2 {
        font-size: 14px; /* ปรับขนาดตัวอักษรหัวข้อ */
    }


}



@media only screen and (max-width: 460px) {

    .note-box {
        width: 100%; /* ปรับให้การ์ดใช้พื้นที่ให้เต็ม */
        max-width: 140px; /* กำหนดขนาดสูงสุด */
        margin: 10px;
    }

    .note-image {
        width: 100%; /* ปรับให้ภาพใช้พื้นที่เต็มการ์ด */
        height: 144px;
    }

    .note-desc {
        font-size: 10px; /* ปรับขนาดตัวอักษรลง */
    }

    .note-desc h2 {
        font-size: 12px; /* ปรับขนาดตัวอักษรหัวข้อ */
    }


}

"#;
