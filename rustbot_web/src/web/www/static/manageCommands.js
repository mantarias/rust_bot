let container = document.querySelector(".commandDisplay");
let commandInput = document.querySelector("#commandInput");
let modal = document.querySelector("#createModal")
let responseTextArea = document.querySelector("#response-textarea");
let modalSaveBtn = document.querySelector("#modal-save-btn");
document.addEventListener("DOMContentLoaded", () => {
    getData();
});
function getData() {
    fetch('/get-commands', {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
    }).then(response => {
        console.log(response.headers.get('Content-Type'));
        console.log(response.headers.get('Date'));
        console.log(response.status);
        console.log(response.statusText);
        console.log(response.type);
        console.log(response.url);
        return response.json()
    })
        .then(data => {
            console.log(data);
            if (Array.isArray(data)) {
                createCommands(data);
            } else if (data.commands && Array.isArray(data.commands)) {
                createCommands(data.commands);
            }
        })
        .catch((error) => {
            console.error('Error:', error);
        });
}

function createCommands(data) {
    data.forEach((el, index) => {
        let card = document.createElement("div");
        card.classList.add("card", "w-96", "bg-neutral", "text-neutral-content", "mb-4", "mt-4");
        card.style.opacity = 0;
        card.style.transition = 'opacity 1s ease';

        setTimeout(() => {
            card.style.opacity = 1;
        }, index * 10);

        let cardBody = document.createElement("div");
        cardBody.classList.add("card-body", "items-center", "text-center");

        let cardTitle = document.createElement("h2");
        cardTitle.classList.add("card-title");
        cardTitle.textContent = "-c " + el.command;

        let cardText = document.createElement("p");
        cardText.textContent = el.response;

        let cardActions = document.createElement("div");
        cardActions.classList.add("card-actions", "justify-end");

        let cardButton = document.createElement("button");
        cardButton.classList.add("btn", "btn-primary");
        cardButton.addEventListener("click", () => {
            modalSaveBtn.dataset.id = index + 1;
            commandInput.value = el.command;
            responseTextArea.value = el.response;
            modal.showModal();

        })
        // cardButton.setAttribute("onclick", "my_modal_2.showModal()");
        cardButton.textContent = "Create New";

        cardActions.appendChild(cardButton);

        cardBody.appendChild(cardTitle);
        cardBody.appendChild(cardText);
        cardBody.appendChild(cardActions);

        card.appendChild(cardBody);

        container.appendChild(card);
    })
}

document.querySelector("#createNewCommand").addEventListener("click", () => {
    commandInput.value = "";
    responseTextArea.value = "";
    modalSaveBtn.dataset.id = "none";
    modal.showModal();
    modalSaveBtn.textContent = "Create";
});


modalSaveBtn.addEventListener("click", () => {
    if (modalSaveBtn.textContent === "Create" && modalSaveBtn.dataset.id === "none") {
        createCommand();
    } else {
        updateCommand();
    }
});


function createCommand() {
    console.log("trying to create command");
    let field1 = commandInput.value;
    let field2 = responseTextArea.value;
    let data = {
        field1,
        field2,
    }
    fetch('/create-command', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    }).then(response => { console.log(response); return response.json() })
        .then(data => {
            console.log('Success:', data);
            location.reload();
        })
        .catch((error) => {
            console.error('Error:', error);
        });
}
function updateCommand() {
    console.log("trying to create command");
    let field1 = commandInput.value;
    let field2 = responseTextArea.value;
    let index = modalSaveBtn.dataset.id;
    let data = {
        field1,
        field2,
        index
    }
    fetch('/update-command', {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    }).then(response => { console.log(response); return response.json() })
        .then(data => {
            console.log('Success:', data);
            location.reload();
        })
        .catch((error) => {
            console.error('Error:', error);
        });
}