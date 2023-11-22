# Remake_Node_Apps_In_Rust

### Non-Blocking IO
Takes in a vector of files, and reads their content to the screen. The files are hard-coded.

### Axum_Learning
Creates an Axum server on localhost:3000, which simply send text. Experimented with GET and POST, sending JSON, and separating functions into multiple files, for scalability <br>
<details>
<summary>Routes:</summary>
<br>
  GET <br>
  <b>'/':</b> Sends text to screen.<br>
  <b>'/about':</b> Sends text to screen.<br>
  <b>'/contact':</b> Sends text to screen.<br>
  <b>'/products':</b> Sends text to screen.<br>
  POST<br>
  <b>'/about/:num':</b> Sends JSON containing 'num'
  <b>'/products':</b> Sends text to screen.<br>
</details>

### EX-01_Math_Api
Creates an Axum server on localhost:3000, which has routes to send text to screen, as well as do simple addition. Learned about Path from the Axum crate, and how to use them. <br>
<details>
  <summary>Routes:</summary>
  <br>
  GET <br>
  <b>'/':</b> Sends text to screen. <br>
  <b>'/about':</b> Sends text to screen. <br>
  <b>'/add/:a/:b':</b> Takes a and b as i32, and adds them, sending the result to the screen.<br>
</details>

### 02_Random_Quotes
Creates an Axum server on localhost:3000, which will get quotes from a JSON file to randomly give to the user. Learned Serde file extraction and deserialization into structs, learned some more about Rust vector interation (filtration, map), learned Axum's HTML return type and Query extraction, as well as how to make it so queries are optional. <br>
<details>
  <summary>Routes:</summary>
  <br>
  GET <br>
  <b>'/quotes':</b> Sends a HTML table of a quote. <br>
  <b>'/quotes?author=[name]&word=[word]':</b> Quotes route contains optional query parameters to allow a user to get quotes from a specific author (case insensitive) or containing a specific word (also case insensitive). The user can use both, one, or none of these.
  <b>'/quotes/:n':</b> Sends n number of quotes to the user, in an HTML table.
</details>

### 03_Weather_Api
Creates an Axum server on localhost:3000, which contains a route that requests a latitude and longitude, so that it can call a fetch to Open Weather Map and return JSON of the data it receieved. I learned about Reqwest, and converting Strings to serde's Value (JSON), though I ended up not using it, since Reqwest can already convert data to a Value. <br>
<details>
  <summary>Routes:</summary>
  <br>
  GET <br>
  <b>'/myweather/:lat/:long':</b> Takes in a latitude and Longitude, as a whole or decimal number, and returns with OpenWeatherMap data for that latitude and Longitude.
</details>

### 04_Cocktails
Created an Axum server on localhost:3000, which contains 2 routes, a simple Hello, World! GET request, and a POST request that allows a user to send JSON of a drink name, to then send to thecocktaildb and parse data on only the drink names, and return an array of those names. Learned about cargo-watch (I like 'cargo-watch -q -x 'run''), and learned more about Serde_Json, such as how to parse through JSON. Added multiple areas of error-checking, such as allowing a user to send no/incorrect data (to where it sends an empty array), and allowed for handling of data that doesn't meet expectations (via json format discrepancies, bad fetch, etc) <br>
<details>
  <summary>Routes:</summary>
  <br>
  GET <br>
  <b>'/test/hello':</b> Sends simple JSON message.<br>
  POST <br>
  <b>'/test/cocktail' | Body: {drink: String}:</b> Takes a JSON object with a 'drink' property and sends that thecocktaildb, and sends the user an array of all cocktails contained in the String.
</details>