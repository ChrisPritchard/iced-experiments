# Weather here

- Tries to get current location, and/or allows coords to be imputed
- Calls a weather api to retrieve info

Only partially completed; currently calls ifconfig.co to retrieve the current lat and long, which achieves what this example helped teach me: how to use the command system with the full iced Application trait.

Next step would be fetching weather, but this requires a subscription to a weather service which I can't (at present) be bothered with

Additionally, ran into issues trying to set border colours for input boxes. Styling here does not work as it did for containers, largely due to the traits like From not being implemented for text_input.