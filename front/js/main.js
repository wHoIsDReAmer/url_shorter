let generate = document.getElementById("shorturl_gen");
generate.onclick = () => {
    let from = document.getElementById("shorturl_from");
    let to = document.getElementById("shorturl_to");

    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/set");
    xhr.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");

    xhr.onload = (e) => {
        let link = window.location.href.split("/").slice(0, -1).join("/");
        // let link = "nx.kro.kr";
        document.getElementById("result").value = link + "/" + from.value;
    };
    xhr.send("from=" + from.value + "&to=" + to.value);
}

document.getElementById("shorturl_cpy").onclick = () => {
    document.getElementById("result").select();
    document.execCommand("copy");
}
