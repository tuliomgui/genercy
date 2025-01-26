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

const toastContainer = document.getElementById('toast-container');

function containerResponse(event) {
    if (event.detail.successful) {
        let jsonResp = JSON.parse(event.detail.xhr.response)
        addToast(jsonResp.message, jsonResp.success ? 'success' : 'error');
    } else {
        addToast("There was an error with the server communication.", 'error');
    }
}

function addToast(message, severity = 'info') {
    if (toastContainer === null) {
        alert("toastContainer is null");
        return;
    }
    const toastModel = toastContainer.getElementsByClassName('toast-model');
    if (toastModel === null || toastModel.length === 0) {
        alert("toastModel is null");
        return;
    }
    const toastClone = toastModel[0].cloneNode(true);
    toastClone.classList.remove('toast-model');
    toastClone.classList.add(getSeverityClass(severity));
    // toastClone.getElementsByTagName('i')[0].classList.add(getSeverityIcon(severity));
    // toastClone.querySelector('.toast-header').textContent = severity.charAt(0).toUpperCase() + severity.slice(1);
    toastClone.querySelector('.toast-body').textContent = message;
    toastContainer.appendChild(toastClone);
    const toast = new bootstrap.Toast(toastClone);
    toast.show();
    toastClone.addEventListener('hidden.bs.toast', function () {
        toastContainer.removeChild(toastClone);
    });
}

function getSeverityClass(severity) {
    switch (severity) {
        case 'success':
            return 'bg-success';
        case 'warning':
            return 'bg-warning';
        case 'error':
            return 'bg-danger';
        default:
            return 'bg-secondary';
    }
}

function getSeverityIcon(severity) {
    switch (severity) {
        case 'success':
            return ['bi', 'bi-check'];
        case 'warning':
            return ['bi', 'bi-exclamation-triangle'];
        case 'error':
            return ['bi', 'bi-x-circle'];
        default:
            return ['bi', 'bi-card-checklist'];
    }
}