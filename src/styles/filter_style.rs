pub const STYLE: &str = r#"
.filter-container {
    display: flex;
    flex-direction: row;
    background-color: #10152A;
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
    align-items: center; /* Aligns checkbox and label vertically */
}

.filter-checkbox {
    width: 20px; /* Width of the checkbox */
    height: 20px; /* Height of the checkbox */
    cursor: pointer; /* Changes cursor on hover */
    -webkit-appearance: none; /* Remove default checkbox appearance */
    appearance: none; /* Remove default checkbox appearance */
    background-color: #151a27; /* Set background color for unchecked checkbox */
    border: 2px solid #1DB954; /* Set border color */
    border-radius: 4px; /* Optional: makes checkbox rounded */
    position: relative;
}

.filter-checkbox:checked {
    background-color: #1DB954; /* Change background color when checked */
    border: 2px solid #1DB954; /* Set border color when checked */
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

