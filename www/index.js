import { trace_rays } from "wasm-ray";

const canvas = document.getElementById("rendered-image");
let im = new ImageData(new Uint8ClampedArray(trace_rays()), 400, 225);
var ctx = canvas.getContext("2d");
ctx.putImageData(im, 0, 0);
