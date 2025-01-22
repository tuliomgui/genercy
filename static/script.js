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