(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/wasm_ray.js":
/*!**************************!*\
  !*** ../pkg/wasm_ray.js ***!
  \**************************/
/*! exports provided: trace_rays */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_ray_bg.wasm */ \"../pkg/wasm_ray_bg.wasm\");\n/* harmony import */ var _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./wasm_ray_bg.js */ \"../pkg/wasm_ray_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"trace_rays\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"trace_rays\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/wasm_ray.js?");

/***/ }),

/***/ "../pkg/wasm_ray_bg.js":
/*!*****************************!*\
  !*** ../pkg/wasm_ray_bg.js ***!
  \*****************************/
/*! exports provided: trace_rays */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"trace_rays\", function() { return trace_rays; });\n/* harmony import */ var _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_ray_bg.wasm */ \"../pkg/wasm_ray_bg.wasm\");\n\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n/**\n* @param {number} width\n* @param {number} height\n* @returns {Uint8Array}\n*/\nfunction trace_rays(width, height) {\n    try {\n        const retptr = _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"trace_rays\"](retptr, width, height);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        var v0 = getArrayU8FromWasm0(r0, r1).slice();\n        _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 1);\n        return v0;\n    } finally {\n        _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n    }\n}\n\n\n\n//# sourceURL=webpack:///../pkg/wasm_ray_bg.js?");

/***/ }),

/***/ "../pkg/wasm_ray_bg.wasm":
/*!*******************************!*\
  !*** ../pkg/wasm_ray_bg.wasm ***!
  \*******************************/
/*! exports provided: memory, trace_rays, __wbindgen_add_to_stack_pointer, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/wasm_ray_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm_ray__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm-ray */ \"../pkg/wasm_ray.js\");\n\n\nconst canvas = document.getElementById(\"rendered-image\"); // as HTMLCanvasElement;\nlet im = new ImageData(new Uint8ClampedArray(Object(wasm_ray__WEBPACK_IMPORTED_MODULE_0__[\"trace_rays\"])(800, 450)), 800, 450);\nvar ctx = canvas.getContext(\"2d\");\nctx.putImageData(im, 0, 0);\n\nconst slider_width = document.getElementById(\"image-width\");\nslider_width.oninput = function() {\n    let imWidth = Number(this.value);\n    let imHeight = Number(slider_height.value);\n\n    // TODO: abstract this all to a separate function\n    canvas.height = imHeight;\n    canvas.width = imWidth;\n\n    let im = new ImageData(new Uint8ClampedArray(Object(wasm_ray__WEBPACK_IMPORTED_MODULE_0__[\"trace_rays\"])(imWidth, imHeight)), imWidth, imHeight);\n    ctx.putImageData(im, 0, 0);\n}\n\nconst slider_height = document.getElementById(\"image-height\");\nslider_height.oninput = function() {\n    let imHeight = Number(this.value);\n    let imWidth = Number(slider_width.value);\n    canvas.height = imHeight;\n    canvas.width = imWidth;\n\n    let im = new ImageData(new Uint8ClampedArray(Object(wasm_ray__WEBPACK_IMPORTED_MODULE_0__[\"trace_rays\"])(imWidth, imHeight)), imWidth, imHeight);\n    ctx.putImageData(im, 0, 0);\n}\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);