// function updateContainerStatus(evt) {
//     if (evt.detail.successful) {
//         //alert("Requested: Status=" + evt.detail.xhr.status + " / ID=" + evt.detail.elt.getAttribute("data-container-id"));
//         let imgStatus = document.getElementById("status-img-" + evt.detail.elt.getAttribute("data-container-id"));
//         if (imgStatus.src.indexOf("red") < 0)
//             imgStatus.src = imgStatus.src.replace("green.svg","red.svg");
//         else
//             imgStatus.src = imgStatus.src.replace("red.svg","green.svg");
//     } else {
//         alert("Error");
//     }
// }

function containerResponse(event) {
    if (event.detail.successful) {
        let jsonResp = JSON.parse(event.detail.xhr.response)
        let success = jsonResp.success;
        let message = jsonResp.message;
        console.log("Success: %o", success);
        console.log("Message: %o", message);
    } else {
        alert("Não foi possível executar a ação desejada, ocorreu um erro de comunicação com o servidor.");
    }
    //console.log("ResponseText: %o", event.detail.xhr.responseText);
}