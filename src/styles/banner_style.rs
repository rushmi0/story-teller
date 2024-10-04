pub const STYLE: &str = r#"

.item-center {
  display: flex;
  justify-content: center;
  align-items: center;
}

#nav {
  background-color: #151a27;
  padding: 1rem;
  display: flex;
  align-items: center;
  justify-content: flex-start;
}

/* ปรับแต่งพิเศษสำหรับขนาดหน้าจอ */

/* สำหรับมือถือ: */
@media only screen and (max-width: 599px) {
    img {
        max-width: auto;
        height: auto; /* ให้ภาพปรับขนาดตามสัดส่วน */
    }
}

/* สำหรับแท็บเล็ตและเดสก์ท็อป: */
@media only screen and (min-width: 768px) {
    #nav {
        justify-content: center;
    }
    img {
        max-width: 200px; /* กำหนดขนาดรูปสำหรับแท็บเล็ตและเดสก์ท็อป */
        height: auto;
    }
}

"#;
