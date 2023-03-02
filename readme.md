# Tridify-rs
### Summary
Work in progress project aiming to provide a really simple but flexible and lightweight low-level GPU rendering framework to create your own frameworks, engines, emulators... You name it!

Currently is under heavy development and is not recommened to be used until some basic features are implemented. Latest cargo releases will aim to be as stable as possible, however there will probably be breaking changes between versions (At least until 1.0 is released). 

### Getting Started
Write ```cargo add tridify-rs``` in your terminal or add ```tridify-rs = "0.2.0"``` to your cargo.toml. 
Also, see examples below to learn the basics.

### Examples
[Here](examples) you can find some examples on how to use Tridify-rs.

### Features
Tridify-rs aims to be low level and flexible to let the user create their own 3D engines and customize them however they want. Any of the features listed below should be feasible using the low-level functionality. 
However I plan on adding more high-level functionality to trade some flexibility for readability and simplification. Here's a list of the planned features.

 - [x] 2D and 3D basic rendering
 - [ ] IMGUI support
 - [ ] Deferred rendering
    - [ ] Lights and Shadows
    - [ ] GPU instancing
 - [ ] Scene framework
 - [ ] Particle and VFXs
 
 Tridify-rs is not meant to be a game engine and won't support other features not related with rendering like input handling, physics, audio or asset management.

