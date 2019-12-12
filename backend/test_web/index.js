
function clicked() {
    console.log('clicked!');
    let request = 'http://localhost:8000/wsa';
    let retJson;
    let xhttp = new XMLHttpRequest();
    let json = {
        no_frames: 15,
        no_processes: 3,
        algorithm: "Opt",
        interval: 3,
        processes: [
            {
                id: 1,
                size: 10,
                requests: '2 4 5 3 6 5 1 3 4 5'
            },
            {
                id: 2,
                size: 10,
                requests: '5 4 6 7 3 4 2 5 5 2'
            },
            {
                id: 3,
                size: 15,
                requests: '6 4 5 3 2 7 8 5 6 8 2 1 3 5 4'
            }
        ]
    }

    xhttp.onreadystatechange = function() {
        if (this.readyState === 4 && this.status === 200) {
            console.log(this.responseText);
            retJson = JSON.parse(this.responseText);
            console.log(retJson);
        }
    }

    xhttp.open('POST', request, true);
    xhttp.send(JSON.stringify(json));
}
