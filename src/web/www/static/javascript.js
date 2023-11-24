let form = document.querySelector("#test");
form.addEventListener("submit", (e) => {
    e.preventDefault();
    fetch('/post', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json', // Ensure correct content type for JSON
        },
        body: JSON.stringify({field1: form.field1.value, field2: form.field2.value}) // Replace with actual data
    }).then(response => {console.log(response); return response.json()}) // Parse JSON response

        .then(data => {
            console.log('Success:', data.message);
            // Handle the data from the server
        })
        .catch((error) => {
            console.error('Error:', error);
        });
})
document.addEventListener("DOMContentLoaded",()=>{
    getData();
});
function getData(){
    fetch('/get-commands', {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json', // Ensure correct content type for JSON
        },
    }).then(response => {console.log(response); return response.json()}) // Parse JSON response

        .then(data => {
            console.log('Success:', data.message);
            // Handle the data from the server
        })
        .catch((error) => {
            console.error('Error:', error);
        });
}

