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
            console.log(error);
        }
    });
}


async function click_button() {
    let olink = document.getElementById("input").value;
    enroll_url(olink);
}

async function input_enter(e) {
    if (e.keyCode == 13) {
        click_button();
    }
}

async function write_result(response) {
    let short_link = "https://" + window.location.host + "/s/" + response.url;
    document.getElementsByClassName("result_box")[0].style.display = "flex";
    document.getElementById("result_text").innerHTML = short_link;
}

async function copy_result() {
    var r = document.createRange();
    r.selectNode(document.getElementById("result_text"));
    window.getSelection().removeAllRanges();
    window.getSelection().addRange(r);
    document.execCommand('copy');
    window.getSelection().removeAllRanges();
}