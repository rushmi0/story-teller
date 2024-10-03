pub const STYLE: &str = r#"

@media only screen and (max-width: 599px) {
    #search-pt {
        margin-left: 30px;
        margin-top: 20px;
        margin-bottom: 20px;
    }

    .search-bar {
        margin-left: 5%;
    }

    .search-bar input {
        width: 100%;
        padding: 0.5rem 1rem;
        border-radius: 8px;
        border: none;
        background-color: #252d42;
        color: white;
        padding-right: 30px; /* เพิ่มพื้นที่ด้านขวาให้กับ input */
    }

    .search-bar input::placeholder {
        color: #8A8F98;
    }
}

@media only screen and (min-width: 600px) {
    #search-pt {
        margin-left: 50px;
        margin-top: 40px;
        margin-bottom: 40px;
    }

    .search-bar {
        margin-left: 20%;
    }

    .search-bar input {
        width: 100%;
        padding: 0.5rem 1rem;
        border-radius: 8px;
        border: none;
        background-color: #252d42;
        color: white;
        padding-right: 30px; /* เพิ่มพื้นที่ด้านขวาให้กับ input */
    }

    .search-bar input::placeholder {
        color: #8A8F98;
    }
}

.input-container {
    position: relative; /* ทำให้ SVG สามารถจัดตำแหน่งแบบ Absolute ได้ */
}

.searchIcon {
    position: absolute;
    right: 30px; /* ปรับตำแหน่งให้อยู่ทางขวาของ input */
    top: 50%; /* จัดตำแหน่งให้อยู่กึ่งกลาง */
    transform: translateY(-50%); /* ปรับให้ SVG อยู่กลางแนวตั้ง */
    width: 13px;
}

.searchIcon path {
    fill: rgb(114, 114, 114);
}

.card {
    border-radius: 20px;
    background: #f5f5f5;
    border: 3px solid #c3c6ce;
    color: #2D323B;
}

.card:hover {
    border-color: #6c00f8;
    box-shadow: 0 0 10px 2px rgba(30, 251, 94, 0.8);
}

"#;
