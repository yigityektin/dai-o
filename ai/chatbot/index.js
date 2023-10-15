const fetch = require("node-fetch");
const readline = require("readline");

const responsesUrl = "https://storage.googleapis.com/replit/responses.json";

let responses;

async function fetchResponses() {
  const response = await fetch(responsesUrl);
  responses = await response.json();
}

function getResponse(message) {
  message = message.toLowerCase();

  for (const key in responses) {
    if (message.includes(key)) {
      return responses[key];
    }
  }

  return "I'm sorry, I don't understand that.";
}

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

async function chatWithBot() {
  await fetchResponses();

  rl.question("You: ", (userInput) => {
    const response = getResponse(userInput);
    console.log("Bot:", response);

    chatWithBot();
  });
}

console.log("Bot: Hi there! How can I help you today?");
chatWithBot();
