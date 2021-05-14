import { memory } from "wasm-ray/wasm_ray_bg";
import { trace_rays } from "wasm-ray";

const canvas = document.getElementById("rendered-image"); // as HTMLCanvasElement;
// let im = new ImageData(new Uint8ClampedArray(trace_rays(800, 450)), 800, 450);
let initial_width = 800;
let initial_height = 450;
const im_ptr = trace_rays(initial_width, initial_height);
let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 4 * initial_height * initial_width), initial_width, initial_height);
var ctx = canvas.getContext("2d");
ctx.putImageData(im, 0, 0);

const slider_width = document.getElementById("image-width");
slider_width.oninput = function() {
    let imWidth = Number(this.value);
    let imHeight = Number(slider_height.value);

    // TODO: abstract this all to a separate function
    canvas.height = imHeight;
    canvas.width = imWidth;
    const im_ptr = trace_rays(imWidth, imHeight);

    let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);
    ctx.putImageData(im, 0, 0);
}

const slider_height = document.getElementById("image-height");
slider_height.oninput = function() {
    let imHeight = Number(this.value);
    let imWidth = Number(slider_width.value);
    canvas.height = imHeight;
    canvas.width = imWidth;
    const im_ptr = trace_rays(imWidth, imHeight);

    let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);
    ctx.putImageData(im, 0, 0);
}
