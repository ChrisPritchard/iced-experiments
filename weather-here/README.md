# Weather here

![](./weatherhere.jpg)

- Tries to get current location (using ifconfig.co), and/or allows coords to be imputed
- Calls a weather api to retrieve info

Instead of calling a weather api (which require an api key usually), uses a mock api that uses thread sleep to simulate time to call. Just like with the coords, this is dispatched using a Command::perform operation, which works really well!

Additionally, ran into issues trying to set border colours for input boxes. Styling here does not work as it did for containers, largely due to the traits like From not being implemented for text_input. I did figure out how to do it by implementing my own stylesheet at [./src/style.rs](./src/style.rs) complete with an Into<TextInput> implementation so it could be passed to .style.