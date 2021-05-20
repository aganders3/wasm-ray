import { memory } from "wasm-ray/wasm_ray_bg";
import { trace_rays_wasm } from "wasm-ray";

const canvas = document.getElementById("rendered-image"); // as HTMLCanvasElement;
const render_time_label = document.getElementById("render-time");
const slider_width = document.getElementById("image-width");
const slider_height = document.getElementById("image-height");
const slider_aa = document.getElementById("anti-aliasing");

function drawImage() {
    let imWidth = Number(slider_width.value);
    let imHeight = Number(slider_height.value);
    let aa = Number(slider_aa.value);

    slider_width.previousSibling.textContent = "Width: " + imWidth;
    slider_height.previousSibling.textContent = "Height: " + imHeight;
    slider_aa.previousSibling.textContent = "Anti-Aliasing: " + aa;

    canvas.height = imHeight;
    canvas.width = imWidth;

    const t0 = performance.now();
    const im_ptr = trace_rays_wasm(imWidth, imHeight, aa);
    let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 3 * imWidth * imHeight), imWidth, imHeight);
    const t1 = performance.now();

    const renderTime = Math.round(t1 - t0);
    render_time_label.textContent = "Render Time: " + renderTime + "ms";

    var ctx = canvas.getContext("2d");
    ctx.putImageData(im, 0, 0);
}

slider_width.oninput = drawImage;
slider_height.oninput = drawImage;
slider_aa.oninput = drawImage;

drawImage();
