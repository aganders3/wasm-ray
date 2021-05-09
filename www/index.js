import { trace_rays } from "wasm-ray";

const canvas = document.getElementById("rendered-image"); // as HTMLCanvasElement;
let im = new ImageData(new Uint8ClampedArray(trace_rays(800, 450)), 800, 450);
var ctx = canvas.getContext("2d");
ctx.putImageData(im, 0, 0);

const slider_width = document.getElementById("image-width");
slider_width.oninput = function() {
    let imWidth = Number(this.value);
    let imHeight = Number(slider_height.value);

    // TODO: abstract this all to a separate function
    canvas.height = imHeight;
    canvas.width = imWidth;

    let im = new ImageData(new Uint8ClampedArray(trace_rays(imWidth, imHeight)), imWidth, imHeight);
    ctx.putImageData(im, 0, 0);
}

const slider_height = document.getElementById("image-height");
slider_height.oninput = function() {
    let imHeight = Number(this.value);
    let imWidth = Number(slider_width.value);
    canvas.height = imHeight;
    canvas.width = imWidth;

    let im = new ImageData(new Uint8ClampedArray(trace_rays(imWidth, imHeight)), imWidth, imHeight);
    ctx.putImageData(im, 0, 0);
}
