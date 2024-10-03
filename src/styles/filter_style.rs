pub const STYLE: &str = r#"
.filter-container {
    display: flex;
    flex-direction: row;
    color: white;
    padding: 1rem;
}

.filter-sidebar {
    width: 200px;
    padding: 1rem;
    background-color: #151a27;
    border-radius: 8px;
    margin-right: 1.5rem;
}

.filter-sidebar h3 {
    font-size: 18px;
    margin-bottom: 10px;
}

.filter-sidebar ul {
    list-style-type: none;
    padding-left: 0;
}

.filter-sidebar li {
    margin-bottom: 10px;
    display: flex;
    align-items: center;
}

.filter-checkbox {
    width: 20px;
    height: 20px;
    cursor: pointer;
    -webkit-appearance: none;
    appearance: none;
    background-color: #151a27;
    border: 2px solid #1DB954;
    border-radius: 4px;
    position: relative;
}

.filter-checkbox:hover {
    box-shadow: 0 0 10px 2px rgba(29, 185, 84, 0.7);
}

.filter-checkbox:checked {
    background-color: #1DB954;
    border: 2px solid #1DB954;
}

.filter-checkbox:checked::after {
    content: '';
    position: absolute;
    top: 2px;
    left: 6px;
    width: 6px;
    height: 10px;
    border: solid white;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
}

.filter-label {
    margin-left: 8px;
    cursor: pointer; /* Makes the label clickable */
}


.header {
    font-size: 16px;
    color: #A7AAB5;
    font-weight: normal;
}

.detail {
    font-size: 16px;
    color: #DBDEE8;
    font-weight: 250;
}

"#;
