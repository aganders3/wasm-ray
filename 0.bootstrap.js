(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/wasm_ray.js":
/*!**************************!*\
  !*** ../pkg/wasm_ray.js ***!
  \**************************/
/*! exports provided: trace_rays, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_ray_bg.wasm */ \"../pkg/wasm_ray_bg.wasm\");\n/* harmony import */ var _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./wasm_ray_bg.js */ \"../pkg/wasm_ray_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"trace_rays\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"trace_rays\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/wasm_ray.js?");

/***/ }),

/***/ "../pkg/wasm_ray_bg.js":
/*!*****************************!*\
  !*** ../pkg/wasm_ray_bg.js ***!
  \*****************************/
/*! exports provided: trace_rays, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"trace_rays\", function() { return trace_rays; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_ray_bg.wasm */ \"../pkg/wasm_ray_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @param {number} width\n* @param {number} height\n* @returns {number}\n*/\nfunction trace_rays(width, height) {\n    var ret = _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"trace_rays\"](width, height);\n    return ret;\n}\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/wasm_ray_bg.js?");

/***/ }),

/***/ "../pkg/wasm_ray_bg.wasm":
/*!*******************************!*\
  !*** ../pkg/wasm_ray_bg.wasm ***!
  \*******************************/
/*! exports provided: memory, trace_rays */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./wasm_ray_bg.js */ \"../pkg/wasm_ray_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/wasm_ray_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm_ray_wasm_ray_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm-ray/wasm_ray_bg */ \"../pkg/wasm_ray_bg.wasm\");\n/* harmony import */ var wasm_ray__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! wasm-ray */ \"../pkg/wasm_ray.js\");\n\n\n\nconst canvas = document.getElementById(\"rendered-image\"); // as HTMLCanvasElement;\n// let im = new ImageData(new Uint8ClampedArray(trace_rays(800, 450)), 800, 450);\nlet initial_width = 800;\nlet initial_height = 450;\nconst im_ptr = Object(wasm_ray__WEBPACK_IMPORTED_MODULE_1__[\"trace_rays\"])(initial_width, initial_height);\nlet im = new ImageData(new Uint8ClampedArray(wasm_ray_wasm_ray_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer, im_ptr, 4 * initial_height * initial_width), initial_width, initial_height);\nvar ctx = canvas.getContext(\"2d\");\nctx.putImageData(im, 0, 0);\n\nconst slider_width = document.getElementById(\"image-width\");\nslider_width.oninput = function() {\n    let imWidth = Number(this.value);\n    let imHeight = Number(slider_height.value);\n\n    // TODO: abstract this all to a separate function\n    canvas.height = imHeight;\n    canvas.width = imWidth;\n    const im_ptr = Object(wasm_ray__WEBPACK_IMPORTED_MODULE_1__[\"trace_rays\"])(imWidth, imHeight);\n\n    let im = new ImageData(new Uint8ClampedArray(wasm_ray_wasm_ray_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);\n    ctx.putImageData(im, 0, 0);\n}\n\nconst slider_height = document.getElementById(\"image-height\");\nslider_height.oninput = function() {\n    let imHeight = Number(this.value);\n    let imWidth = Number(slider_width.value);\n    canvas.height = imHeight;\n    canvas.width = imWidth;\n    const im_ptr = Object(wasm_ray__WEBPACK_IMPORTED_MODULE_1__[\"trace_rays\"])(imWidth, imHeight);\n\n    let im = new ImageData(new Uint8ClampedArray(wasm_ray_wasm_ray_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);\n    ctx.putImageData(im, 0, 0);\n}\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);