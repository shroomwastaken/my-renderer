# my-renderer
tiny program that renders a 3D cube with sdl2

## building
1. clone the repo
2. follow the rust-sdl2 installation guide for your os [here](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries)
3. `cargo build --release`
4. profit

## how
1. take unit cube defined in 3D space
2. define 2 triangles for each face (make sure the points in those triangles are defined clockwise)
3. apply some transformations at will (rotations, translations, etc.)
3. project the points of these triangles onto 2D space using the perspective projection matrix
4. draw triangles to screen with sdl2
5. profit

i will admit some of the linear algebra stuff kinda goes over my head but i think it's fine to treat it as a "black box" of sorts

## used sources
[this](https://www.youtube.com/watch?v=ih20l3pJoeU) tutorial by javidx9