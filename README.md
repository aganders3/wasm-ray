# Ray tracing in Rust and WebAssembly

![Cover Image](/renders/cover_image.png)

~~A live demo of the current code should be available at https://aga3.xyz/wasm-ray~~

*The wasm-based live demo is probably not working for now.*

I pretty quickly ran into performance issues with the wasm version. I have not
been able to successfully parallelize that version, and I'm not sure it's even
possible. I may try to revisit it, or at least to enable it with some hard
limits on resolution, etc. to make it run in reasonable time. It might not be
that bad if I can also get a progress bar working or something...

This is a project to help me learn more about:
* Ray tracing
* Rust
* WebAssembly (aka wasm)


Thanks to (very seriously in no particular order):
* this amazing series on ray tracing by Peter Shirley: https://raytracing.github.io
* this awesome project for the template: https://github.com/rustwasm/wasm-pack-template
* this great tutorial for Rust + WebAssembly: https://rustwasm.github.io/docs/book/game-of-life/introduction.html
