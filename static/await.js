const AWAIT_SYNTAXES = [
    "await yet?",
    "(await yet)?",
    "await!(yet)?",
    "await? yet",
    "await { yet }?",
    "yet.await?",
    "yet.await()?",
    "yet.await!()?",
    "yet await?",
    "yet!?",
    "yet#?",
    "yet@?",
];
document.getElementById("awaityet").innerHTML = AWAIT_SYNTAXES[Math.floor(Math.random() * AWAIT_SYNTAXES.length)];

let dates = document.querySelectorAll(".relative");
for (let i = 0; i < dates.length; i++) {
    let m = moment(dates[i].getAttribute("data-date"));
    dates[i].innerHTML = m.fromNow();
}
