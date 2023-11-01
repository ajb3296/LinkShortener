async function enroll_url(olink) {
    $.ajax({
        url: '/enroll/',
        type: 'POST',
        dataType: 'JSON',
        async: true,
        data: JSON.stringify({ "url": olink }),
        contentType: "application/json",
        success: function(response) {
            write_result(response);
        },
        error: function(error) {
            alert("Error");
            document.getElementsByClassName("result_box")[0].style.display = "none";
            setTimeout(() => document.getElementById("submit_button").disabled = false, 3000);
        }
    });
}


async function click_button() {
    document.getElementById("submit_button").disabled = true;
    let olink = document.getElementById("input").value;
    enroll_url(olink);
}

async function input_enter(e) {
    if (e.keyCode == 13 && !document.getElementById("submit_button").disabled) {
        click_button();
    }
}

async function write_result(response) {
    let short_link = window.location.protocol + "//" + window.location.host + "/s/" + response.url;
    document.getElementsByClassName("result_box")[0].style.display = "flex";
    document.getElementById("result_text").innerHTML = short_link;

    setTimeout(() => document.getElementById("submit_button").disabled = false, 3000);
}

async function copy_result() {
    var r = document.createRange();
    r.selectNode(document.getElementById("result_text"));
    window.getSelection().removeAllRanges();
    window.getSelection().addRange(r);
    document.execCommand('copy');
    window.getSelection().removeAllRanges();

    alert("복사되었습니다");
}