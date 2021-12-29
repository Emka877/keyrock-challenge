console.log("main.js loaded.");

// let queryDataButton = document.getElementById("btn-query-data");
let asksData = document.getElementById("asks-data");
let bidsData = document.getElementById("bids-data");
let spreadData = document.getElementById("spread-data");

const QUERY_INTERVAL_MS = 200;
let query_timeout_object = null;

function initialize() {
    // queryDataButton.onclick = ((evt) => {
    //     evt.preventDefault();
    //     getQuotesFromServer();
    // });

    getQuotesFromServer();
    query_timeout_object = setInterval(() => getQuotesFromServer(), QUERY_INTERVAL_MS);
}

function quoteObjectToDOM(quote) {
    let element = document.createElement("div");
    
    // Sub elements
    // Alternate method: Iterate over object keys and create a <span> for each key, then put value in innerHTML
    let exchange = document.createElement("div");
    exchange.className = "exchange";
    exchange.innerHTML = quote.exchange;
    let price = document.createElement("div");
    price.className = "price";
    price.innerHTML = quote.price;
    let amount = document.createElement("div");
    amount.className = "amount";
    amount.innerHTML = quote.amount;
    
    // Merge it all
    element.appendChild(exchange);
    element.appendChild(price);
    element.appendChild(amount);
    
    return element;
}

function getQuotesFromServer() {
    fetch("http://keyrock-visual.oscur.io:50006/get_quotes")
        .then(data => data.json())
        .then(res => {
            if (res.noDataCached) {
                console.error("Response received from server, but data is empty!");
                return;
            }
            
            // console.log(res.data);
            
            // asksData.innerHTML = JSON.stringify(res.data.asks);
            // bidsData.innerHTML = JSON.stringify(res.data.bids);
            asksData.innerHTML = "";
            res.data.asks.forEach(quote => {
                let dom = quoteObjectToDOM(quote).innerHTML;
                asksData.innerHTML += "<div class='quotes-data-root'>" + dom + "</div>";
            });

            bidsData.innerHTML = "";
            res.data.bids.forEach(quote => {
                let dom = quoteObjectToDOM(quote).innerHTML;
                bidsData.innerHTML += "<div class='quotes-data-root'>" + dom + "</div>";
            });

            spreadData.innerHTML = res.data.spread;
        })
        .catch(err => console.error(err));
}

function main() {
    initialize();
}

main();
