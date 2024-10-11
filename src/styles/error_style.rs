pub const STYLE: &str = r#"
html, body {
    margin: 0;
    padding: 0;
    height: 100%;
}

body {
    background-color: #F4F4F4;
    display: flex;
    justify-content: center;
    align-items: center;
}


.basic-style {
    height: 100%;
    margin: 0;
    font-size: 62.5%;
    color: #293b49;
}

.center {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
}

.error {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-content: center;
}

.number {
    font-weight: 900;
    font-size: 15rem;
    line-height: 1;
}

.illustration {
    position: relative;
    width: 12.2rem;
    margin: 0 2.1rem;
}

.circle {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 12.2rem;
    height: 11.4rem;
    border-radius: 50%;
    background-color: #293b49;
}

.clip {
    position: absolute;
    bottom: 0.3rem;
    left: 50%;
    transform: translateX(-50%);
    overflow: hidden;
    width: 12.5rem;
    height: 13rem;
    border-radius: 0 0 50% 50%;
}

.paper {
    position: absolute;
    bottom: -0.3rem;
    left: 50%;
    transform: translateX(-50%);
    width: 9.2rem;
    height: 12.4rem;
    border: 0.3rem solid #293b49;
    background-color: white;
    border-radius: 0.8rem;
}

.paper::before {
    content: "";
    position: absolute;
    top: -0.7rem;
    right: -0.7rem;
    width: 1.4rem;
    height: 1rem;
    background-color: white;
    border-bottom: 0.3rem solid #293b49;
    transform: rotate(45deg);
}

.face {
    position: relative;
    margin-top: 2.3rem;
}

.eyes {
    position: absolute;
    top: 0;
    left: 2.4rem;
    width: 4.6rem;
    height: 0.8rem;
}

.eye {
    position: absolute;
    bottom: 0;
    width: 0.8rem;
    height: 0.8rem;
    border-radius: 50%;
    background-color: #293b49;
    animation-name: eye;
    animation-duration: 4s;
    animation-iteration-count: infinite;
    animation-timing-function: ease-in-out;
}

.eye-left {
    left: 0;
}

.eye-right {
    right: 0;
}

@keyframes eye {
    0%, 50%, 54%, 100% {
        height: 0.8rem;
    }
    52% {
        height: 0.1rem;
    }
}

.rosyCheeks {
    position: absolute;
    top: 1.6rem;
    width: 1rem;
    height: 0.2rem;
    border-radius: 50%;
    background-color: #fdabaf;
}

.rosyCheeks-left {
    left: 1.4rem;
}

.rosyCheeks-right {
    right: 1.4rem;
}

.mouth {
    position: absolute;
    top: 3.1rem;
    left: 50%;
    width: 1.6rem;
    height: 0.2rem;
    border-radius: 0.1rem;
    transform: translateX(-50%);
    background-color: #293b49;
}

.text {
    margin-top: 5rem;
    font-weight: 300;
    color: #293b49;
}

.button {
    margin-top: 4rem;
    padding: 1.2rem 3rem;
    color: white;
    background-color: #04cba0;
}

.button:hover {
    background-color: #01ac88;
}

.by {
    position: absolute;
    bottom: 0.5rem;
    left: 0.5rem;
    text-transform: uppercase;
    color: #293b49;
}

.byLink {
    color: #04cba0;
}


@media only screen and (max-width: 768px) {
    .number {
        font-size: 8vw;
    }

    .illustration {
        width: 12vw;
    }

    .circle {
        width: 12vw;
        height: 11vw;
    }

    .paper {
        width: 9vw;
        height: 12vw;
    }

    .text {
        font-size: 1.2rem;
    }

    .button {
        padding: 0.8rem 2rem;
        font-size: 1rem;
    }
}

@media only screen and (max-width: 480px) {
    .number {
        font-size: 6vw;
    }

    .illustration {
        width: 10vw;
    }

    .circle {
        width: 10vw;
        height: 9vw;
    }

    .paper {
        width: 7vw;
        height: 10vw;
    }

    .text {
        font-size: 1rem;
    }

    .button {
        padding: 0.6rem 1.5rem;
        font-size: 0.9rem;
    }
}

"#;
