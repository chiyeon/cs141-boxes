![](preview/preview1.png)

# Boxes
**Boxes** is a very crude voxel engine, made for CS141's Extra Credit project opportunity.

Written in Rust and powered by the [Macroquad](https://macroquad.rs/) game engine, it generates a simple island using perlin noise, populated with beaches and trees. Islands are randomly generated on startup.

Note that this implementation of a voxel engine *isn't optimal.* The only optimzations it has is to hide non-visible faces. A better engine would have face meshing and so on.

![](preview/preview2.png)
