function updateContainerStatus(containerId) {
    let containerLine = document.getElementById("line-" + containerId);
    const badge = containerLine.getElementsByClassName("badge")[0];
    const controls = containerLine.getElementsByClassName("container-controls")[0];
    if (badge.classList.contains('badge-success')) {
        badge.parentNode.innerHTML = '<span class="badge badge-danger">Stopped</span>';
        controls.classList.remove("state-started");
        controls.classList.add("state-stopped");
    } else {
        badge.parentNode.innerHTML = '<span class="badge badge-success">Running</span>';
        controls.classList.remove("state-stopped");
        controls.classList.add("state-started");
    }
}

const toastContainer = document.getElementById('toast-container');

function containerResponse(event) {
    if (event.detail.successful) {
        let jsonResp = JSON.parse(event.detail.xhr.response)
        addToast(jsonResp.message, jsonResp.success ? 'success' : 'error');
        if (jsonResp.success) {
            updateContainerStatus(jsonResp.id);
        }
    } else {
        addToast("There was an error with the server communication.", 'error');
    }
}

function addClasses(element, classes) {
    classes.forEach(cls => element.classList.add(cls));
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
    addClasses(toastClone.getElementsByTagName('i')[0], getSeverityIcon(severity));
    toastClone.querySelector('.toast-header-text').textContent = severity.charAt(0).toUpperCase() + severity.slice(1);
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
            return 'text-bg-success';
        case 'warning':
            return 'text-bg-warning';
        case 'error':
            return 'text-bg-danger';
        default:
            return 'text-bg-secondary';
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