# Weather here

- Tries to get current location, and/or allows coords to be imputed
- Calls the metservice api https://www.metservice.com/point-forecast-api/plans to get weather details and prints on the screen.

Model would be the weather, perhaps some stats like humidity, rain etc, and would allow you to specify time jumping between today, tomorrow etc.

Events therefore would be:

- FetchCoords
- SetCoords
- FetchWeather
- WeatherReceived