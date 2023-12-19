# Remake_Node_Apps_In_Rust
The goal of this repo is to recreate all my apps created in https://github.com/Naginipython/Scalable_Web_Systems into Rust

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
Created an Axum server on localhost:3000, which contains 2 routes, a simple Hello, World! GET request, and a POST request that allows a user to send JSON of a drink name, to then send to thecocktaildb and parse data on only the drink names, and return an array of those names. Learned about cargo-watch (I like 'cargo-watch -q -x 'run''), and learned more about Serde_Json, such as how to parse through JSON. Added multiple areas of error-checking, such as allowing a user to send no/incorrect data (to where it sends an empty array), and allowed for handling of data that doesn't meet expectations (via json format discrepancies, bad fetch, etc). I took time later to figure out middleware, and ran into issues where axum wasn't updated to suit tower-http. Figured out getting basic request logging, only accepting JSON as a header, and cors.<br>
<details>
  <summary>Routes:</summary>
  <br>
  GET <br>
  <b>'/test/hello':</b> Sends simple JSON message.<br>
  POST <br>
  <b>'/test/cocktail' | Body: {drink: String}:</b> Takes a JSON object with a 'drink' property and sends that thecocktaildb, and sends the user an array of all cocktails contained in the String.
</details>
<<<<<<< HEAD

### 05_Caesar-Cypher
Created an Axum server on localhost:3000, which contains 4 routes, 2 simple 'hello' POST requests, for some testing, and 2 POST requests for encrypting and decrypting, 'testEncrypt' and 'testDecrypt'. This was a particularily troublesome exercise, as Axum had just updated to implement better middleware features, but I struggled to figure out how to use Tower and Http with Axum, as there wasn't very detailed documentation. I did en up figuring it out, and implemented 'my_middleware', which would go between every route but only change data if the routes are 'testEncrypt' and 'testDecrypt'. Here, a user would add a shift in the path, as well as send JSON in the body, and the middleware would take both of those and encrypt/decrypt a word in the JSON, as well as append a time and date. Afterwards, it would continue sending requests through the layers an it would, until it reaches the POSTs. I learned how to create my own Error responses, how to get a body from an incoming request (through it's bytes), I learned how to modify headers (I need to change header's content-length), and I used a cool in-line function closure to modify char_codes for the en/decryption (the_math variable). <br>
<details>
  <summary>Routes:</summary>
  <br>
  POST <br>
  <b>'/testEncrypt/:shift' | Body: {word: String}:</b> Takes a JSON object with a 'word' property, and a positive shift value in the path, and returns the 'word' property word, shifted alphabetically by the shift number. It also contains new fields, a 'date' of the current date, and 'time', of the current time.<br>
  <b>'/testDecrypt/:shift' | Body: {word: String}:</b> Takes a JSON object with a 'word' property, and a positive shift value in the path, and returns the 'word' property word, shifted alphabetically by the shift number. It also contains new fields, a 'date' of the current date, and 'time', of the current time.<br>
  <b>'/hello/':</b> A POST request that simply sends text.<br>
  <b>'/helloeveryoneintheworld/':</b> A POST request that simply sends text.<br>
</details>
=======
>>>>>>> 0a18eae26283c9ab93b7a229acff7b83d60dc0ba
