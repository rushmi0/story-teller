pub const STYLE: &str = r#"

.card-container {
    flex-wrap: wrap;
    display: flex;
    justify-content: center;
}

.card {
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
  width: 252px;
  margin: 10px;
}



.card-out:hover {
  transform: scale(1.05);
  border: 3px solid #f7f7f7;
}

.card-image {
  width: 252px;
  height: 256px;
  overflow: hidden;
  object-fit: cover;
  border-radius: 15px 15px 0px 0px;
}

.card-text {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-desc {
  padding-top: 12px;
  padding-left: 10px;
  padding-right: 10px;
  font-size: 16px;
  color: #a7aab5;
  line-height: 16px;
}

.card-desc h2 {
  color: #f7f7f7;
  font-size: 18px;
}

.card-icon {
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


"#;
