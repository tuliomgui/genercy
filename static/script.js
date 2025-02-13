function updateContainerStatus(containerId) {
    let containerLine = document.getElementById("line-" + containerId);
    const badge = containerLine.getElementsByClassName("badge")[0];
    const controls = containerLine.getElementsByClassName("container-controls")[0];
    if (badge.classList.contains('badge-success')) {
        badge.parentNode.innerHTML = '<span class="badge badge-danger">Stopped</span>';
        controls.classList.remove("state-stopped");
        controls.classList.add("state-started");
    } else {
        badge.parentNode.innerHTML = '<span class="badge badge-success">Running</span>';
        controls.classList.remove("state-started");
        controls.classList.add("state-stopped");
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

function handleImageDeletion(event) {
    if (event.detail.successful) {
        let jsonResp = JSON.parse(event.detail.xhr.response)
        addToast(jsonResp.message, jsonResp.success ? 'success' : 'error');
        if (jsonResp.success) {
            removeTableRow('gen-images-tbody', 'gen-line-'+jsonResp.id);
        }
    } else {
        addToast("There was an error with the server communication.", 'error');
    }
}

function removeTableRow(tbodyId, rowId) {
    const tbody = document.getElementById(tbodyId);
    const row = document.getElementById(rowId);
    if (tbody && row) {
        tbody.removeChild(row);
        const rows = tbody.getElementsByTagName('tr');
        for (let i = 0; i < rows.length; i++) {
            const rowNumberCell = rows[i].getElementsByClassName('tbody-row-number')[0];
            if (rowNumberCell) {
                rowNumberCell.textContent = i + 1;
            }
        }
    }
}

function openModal(modalData, afterReqFunc) {
    const modal = document.getElementById(modalData.id);
    const modalHeader = modal.getElementsByClassName('modal-title')[0];
    const modalBody = modal.getElementsByClassName('modal-body')[0];
    const modalButton = modal.getElementsByClassName('modal-button')[0];
    modalHeader.textContent = modalData.title;
    modalBody.textContent = modalData.message;
    modalButton.textContent = modalData.buttonText;
    modalButton.classList.remove('btn-primary', 'btn-danger', 'btn-success');
    modalButton.classList.add(modalData.buttonClass);
    //modalButton.dataset.hxDelete = modalData.reqUrl;
    modalButton.dataset.hxGet = "/hello2";
    modalButton.dataset.hxSwap = 'none';
    modalButton.setAttribute('hx-on:htmx:after-request', afterReqFunc.name + '(event)');
    htmx.process(modal);
    (new bootstrap.Modal(modal)).show();
}

function tempTeste(event) {
    console.log('Evento: ', event);
    let jsonResp = JSON.parse(event.detail.xhr.response)
    console.log("Mensagem: ", jsonResp.message);
}