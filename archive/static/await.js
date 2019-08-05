const AWAIT_SYNTAXES = [
    "await anymore",
    "await!(anymore)",
    "await anymore",
    "await { anymore }",
    "anymore.await()",
    "anymore.await!()",
    "anymore await",
    "anymore!",
    "anymore#",
    "anymore@",
];
document.getElementById("awaityet").innerHTML = AWAIT_SYNTAXES[Math.floor(Math.random() * AWAIT_SYNTAXES.length)];

let dates = document.querySelectorAll(".relative");
for (let i = 0; i < dates.length; i++) {
    let m = moment(dates[i].getAttribute("data-date"));
    dates[i].innerHTML = m.fromNow();
}
