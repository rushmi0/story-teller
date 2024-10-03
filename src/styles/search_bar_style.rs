pub const STYLE: &str = r#"
.search-bar {
    flex-grow: 1;
    display: flex;
    justify-content: flex-start;
    align-items: center;
}

.search-bar input {
    width: 100%;
    padding: 0.5rem 1rem;
    border-radius: 8px;
    border: none;
    background-color: #252d42;
    color: white;
}

.search-bar input::placeholder {
    color: #8A8F98;
}
"#;