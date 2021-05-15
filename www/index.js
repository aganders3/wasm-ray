import { memory } from "wasm-ray/wasm_ray_bg";
import { trace_rays } from "wasm-ray";

const canvas = document.getElementById("rendered-image"); // as HTMLCanvasElement;
// let im = new ImageData(new Uint8ClampedArray(trace_rays(800, 450)), 800, 450);
let initial_width = 400;
let initial_height = 225;
let initial_aa = 1;
const im_ptr = trace_rays(initial_width, initial_height, initial_aa);
let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 4 * initial_height * initial_width), initial_width, initial_height);
var ctx = canvas.getContext("2d");
ctx.putImageData(im, 0, 0);

const slider_width = document.getElementById("image-width");
const slider_height = document.getElementById("image-height");
const slider_aa = document.getElementById("anti-aliasing");

slider_width.oninput = function() {
    let imWidth = Number(this.value);
    let imHeight = Number(slider_height.value);
    let aa = Number(slider_aa.value);

    // TODO: abstract this all to a separate function
    canvas.height = imHeight;
    canvas.width = imWidth;
    const im_ptr = trace_rays(imWidth, imHeight, aa);

    let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);
    ctx.putImageData(im, 0, 0);
}

slider_height.oninput = function() {
    let imHeight = Number(this.value);
    let imWidth = Number(slider_width.value);
    let aa = Number(slider_aa.value);

    canvas.height = imHeight;
    canvas.width = imWidth;
    const im_ptr = trace_rays(imWidth, imHeight, aa);

    let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);
    ctx.putImageData(im, 0, 0);
}

slider_aa.oninput = function() {
    let imHeight = Number(slider_height.value);
    let imWidth = Number(slider_width.value);
    let aa = Number(this.value);

    canvas.height = imHeight;
    canvas.width = imWidth;
    const im_ptr = trace_rays(imWidth, imHeight, aa);

    let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);
    ctx.putImageData(im, 0, 0);
}
