let container = document.querySelector(".commandDisplay");
document.addEventListener("DOMContentLoaded", ()=>{
    getData();
});
function getData(){
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
            return response.json()})
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

function createCommands(data){
    data.forEach((el)=>{
        let card = document.createElement("div");
        card.classList.add("card", "w-96", "bg-neutral", "text-neutral-content", "mb-4", "mt-4");

        let cardBody = document.createElement("div");
        cardBody.classList.add("card-body", "items-center", "text-center");

        let cardTitle = document.createElement("h2");
        cardTitle.classList.add("card-title");
        cardTitle.textContent = el.command;

        let cardText = document.createElement("p");
        cardText.textContent = el.response;

        let cardActions = document.createElement("div");
        cardActions.classList.add("card-actions", "justify-end");

        let cardButton = document.createElement("button");
        cardButton.classList.add("btn", "btn-primary");
        cardButton.setAttribute("onclick", "my_modal_2.showModal()");
        cardButton.textContent = "Create New";

        cardActions.appendChild(cardButton);

        cardBody.appendChild(cardTitle);
        cardBody.appendChild(cardText);
        cardBody.appendChild(cardActions);

        card.appendChild(cardBody);

        container.appendChild(card);
    })
}