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
  justify-content: flex-start; /* Default for mobile */
}

/* For desktop: */
@media only screen and (min-width: 768px) {
  #nav {
    justify-content: center;
  }
}

"#;
