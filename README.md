# Septoria
<p align="center">
<img src="https://agrilife.org/plantdiseasehandbook/files/2014/06/1570809-PPT.jpg"
width="300"
/>
</p>


A create for interacting with the [lemon.markets](https://www.lemon.markets/) API.

## For developers

The project won't compile without sufficient documentation.

This Repository follows the [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) convention for commits.

Make sure that you follows there conventions when you make a commit. Every PR that does
not follow the conventional commit convention, and which does not compile doe to missing docs will be denied.

### Setting up your local development environment

First, clone the project:
```bash
  git clone https://github.com/Uzaaft/Septoria
```

Go to the project directory

```bash
  cd septoria
```
And set up pre-commits if you'd like. Follow the official pre-commit docs for a guide. 

### Tests

To run the test, make sure you have a
`.env` file with the following variables defined:

```
LEMON_MARKET_TRADING_API_KEY=<MARKET_API_KEY_HERE>
```

I recommend that you use [nextest](https://nexte.st/) to run the tests.