# my-renderer
tiny program that is capable of rendering 3D models with sdl2 and sdl2_gfx

only .obj files are supported

## building
1. clone the repo
2. follow the rust-sdl2 installation guide for your os [here](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries)
3. do the same thing you did for normal sdl2 but with [sdl2_gfx](https://github.com/giroletm/SDL2_gfx/releases/tag/release-1.0.4)
4. `cargo build --release`
5. profit

## how
1. take model
2. get its faces (triangles)
3. apply some transformations at will (rotations, translations, etc.)
4. project the points of these faces onto 2D space using the perspective projection matrix
5. draw faces to screen with sdl2
6. profit

i will admit some of the linear algebra stuff kinda goes over my head but i think it's fine to treat it as a "black box" of sorts

## usage
`cargo run --release <filename> <distance from camera>`

only the most primitive obj files are supported, your fancy model might not work

if the model is clipping inside the camera increase the distance

## used sources
[this](https://www.youtube.com/watch?v=ih20l3pJoeU) tutorial by javidx9