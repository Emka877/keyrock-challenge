console.log("main.js loaded.");

// DOM Elements
let asksData = document.getElementById("asks-data");
let bidsData = document.getElementById("bids-data");
let spreadData = document.getElementById("spread-data");

// "Enums"
const E_ASKS = "ask";
const E_BIDS = "bid";

// Data cache
let currentAsks = null;
let currentBids = null;
let previousAsks = null;
let previousBids = null;
let previousSpread = null;
let currentSpread = null;

// Vars
const QUERY_INTERVAL_MS = 1000;
let query_timeout_object = null;

function initialize() {
    getQuotesFromServer();
    query_timeout_object = setInterval(() => getQuotesFromServer(), QUERY_INTERVAL_MS);
}

function compareFloats(a, b) {
    let difference = a - b;
    if (difference < 0.0) {
        return -1;
    } else if (difference > 0.0) {
        return 1;
    } else {
        return 0;
    }
}

function compareQuotesPrice(current, previous) {
    return compareFloats(current.price, previous.price);
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
    price.innerHTML = quote.price.toFixed(8);
    let amount = document.createElement("div");
    amount.className = "amount";
    amount.innerHTML = quote.amount.toFixed(8);
    
    // Merge it all
    element.appendChild(exchange);
    element.appendChild(price);
    element.appendChild(amount);
    
    return element;
}

function getQuotesFromServer() {
    fetch("http://127.0.0.1:50006/get_quotes")
        .then(data => data.json())
        .then(res => {
            if (res.noDataCached) {
                console.error("Response received from server, but data is empty!");
                return;
            }

            // Update data cache
            previousAsks = currentAsks;
            previousBids = currentBids;
            currentAsks = res.data.asks;
            currentBids = res.data.bids;
            previousSpread = currentSpread;
            currentSpread = res.data.spread;
            
            asksData.innerHTML = "";
            currentAsks.forEach((quote, idx) => {
                let previousQuoteSameSlot = null;
                let comparisonResult = 0;
                if (previousAsks) {
                    previousQuoteSameSlot = previousAsks[idx];
                    comparisonResult = compareQuotesPrice(quote, previousQuoteSameSlot);
                }
                
                let dom = quoteObjectToDOM(quote).innerHTML;
                let classes = "quotes-data-root";
                
                if (comparisonResult === 1) {
                    classes += " upwards";
                } else if (comparisonResult === -1) {
                    classes += " downwards";
                }
                
                asksData.innerHTML += "<div class='" + classes + "'> " + dom + "</div>";
            });

            bidsData.innerHTML = "";
            currentBids.forEach((quote, idx) => {
                let previousQuoteSameSlot = null;
                let comparisonResult = 0;
                if (previousAsks) {
                    previousQuoteSameSlot = previousBids[idx];
                    comparisonResult = compareQuotesPrice(quote, previousQuoteSameSlot);
                }

                let dom = quoteObjectToDOM(quote).innerHTML;
                let classes = "quotes-data-root";
                
                if (comparisonResult === 1) {
                    classes += " upwards";
                } else if (comparisonResult === -1) {
                    classes += " downwards";
                }
                
                bidsData.innerHTML += "<div class='" + classes + "'> " + dom + "</div>";
            });

            const spreadCompare = compareFloats(currentSpread, previousSpread);
            let classes = "";
            if (spreadCompare === 1) {
                classes = "upwards";
            } else if (spreadCompare === -1) {
                classes = "downwards";
            }
            spreadData.innerHTML = "<span class='" + classes + "'>" + currentSpread + "</span>";
        })
        .catch(err => console.error(err));
}

function main() {
    initialize();
}

main();
