(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/wasm_ray.js":
/*!**************************!*\
  !*** ../pkg/wasm_ray.js ***!
  \**************************/
/*! exports provided: trace_rays, __wbindgen_object_drop_ref, __wbg_getRandomValues_57e4008f45f0e105, __wbg_randomFillSync_d90848a552cbd666, __wbg_static_accessor_MODULE_39947eb3fe77895f, __wbg_self_f865985e662246aa, __wbg_require_c59851dfa0dc7e78, __wbg_crypto_bfb05100db79193b, __wbg_msCrypto_f6dddc6ae048b7e2, __wbindgen_is_undefined, __wbg_buffer_ebc6c8e75510eae3, __wbg_new_135e963dedf67b22, __wbg_set_4a5072a31008e0cb, __wbg_length_317f0dd77f7a6673, __wbg_newwithlength_78dc302d31527318, __wbg_subarray_34c228a45c72d146, __wbindgen_throw, __wbindgen_memory */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_ray_bg.wasm */ \"../pkg/wasm_ray_bg.wasm\");\n/* harmony import */ var _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./wasm_ray_bg.js */ \"../pkg/wasm_ray_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"trace_rays\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"trace_rays\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_57e4008f45f0e105\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_57e4008f45f0e105\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_d90848a552cbd666\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_randomFillSync_d90848a552cbd666\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_static_accessor_MODULE_39947eb3fe77895f\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_static_accessor_MODULE_39947eb3fe77895f\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_f865985e662246aa\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_self_f865985e662246aa\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_c59851dfa0dc7e78\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_require_c59851dfa0dc7e78\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_bfb05100db79193b\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_crypto_bfb05100db79193b\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_msCrypto_f6dddc6ae048b7e2\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_msCrypto_f6dddc6ae048b7e2\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_ebc6c8e75510eae3\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_buffer_ebc6c8e75510eae3\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_135e963dedf67b22\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_135e963dedf67b22\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_4a5072a31008e0cb\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_set_4a5072a31008e0cb\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_length_317f0dd77f7a6673\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_length_317f0dd77f7a6673\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithlength_78dc302d31527318\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_newwithlength_78dc302d31527318\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_subarray_34c228a45c72d146\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_subarray_34c228a45c72d146\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return _wasm_ray_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_memory\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/wasm_ray.js?");

/***/ }),

/***/ "../pkg/wasm_ray_bg.js":
/*!*****************************!*\
  !*** ../pkg/wasm_ray_bg.js ***!
  \*****************************/
/*! exports provided: trace_rays, __wbindgen_object_drop_ref, __wbg_getRandomValues_57e4008f45f0e105, __wbg_randomFillSync_d90848a552cbd666, __wbg_static_accessor_MODULE_39947eb3fe77895f, __wbg_self_f865985e662246aa, __wbg_require_c59851dfa0dc7e78, __wbg_crypto_bfb05100db79193b, __wbg_msCrypto_f6dddc6ae048b7e2, __wbindgen_is_undefined, __wbg_buffer_ebc6c8e75510eae3, __wbg_new_135e963dedf67b22, __wbg_set_4a5072a31008e0cb, __wbg_length_317f0dd77f7a6673, __wbg_newwithlength_78dc302d31527318, __wbg_subarray_34c228a45c72d146, __wbindgen_throw, __wbindgen_memory */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"trace_rays\", function() { return trace_rays; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_57e4008f45f0e105\", function() { return __wbg_getRandomValues_57e4008f45f0e105; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_d90848a552cbd666\", function() { return __wbg_randomFillSync_d90848a552cbd666; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_static_accessor_MODULE_39947eb3fe77895f\", function() { return __wbg_static_accessor_MODULE_39947eb3fe77895f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_f865985e662246aa\", function() { return __wbg_self_f865985e662246aa; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_c59851dfa0dc7e78\", function() { return __wbg_require_c59851dfa0dc7e78; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_bfb05100db79193b\", function() { return __wbg_crypto_bfb05100db79193b; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_msCrypto_f6dddc6ae048b7e2\", function() { return __wbg_msCrypto_f6dddc6ae048b7e2; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_ebc6c8e75510eae3\", function() { return __wbg_buffer_ebc6c8e75510eae3; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_135e963dedf67b22\", function() { return __wbg_new_135e963dedf67b22; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_4a5072a31008e0cb\", function() { return __wbg_set_4a5072a31008e0cb; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_length_317f0dd77f7a6673\", function() { return __wbg_length_317f0dd77f7a6673; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithlength_78dc302d31527318\", function() { return __wbg_newwithlength_78dc302d31527318; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_subarray_34c228a45c72d146\", function() { return __wbg_subarray_34c228a45c72d146; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return __wbindgen_memory; });\n/* harmony import */ var _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_ray_bg.wasm */ \"../pkg/wasm_ray_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n/**\n* @param {number} width\n* @param {number} height\n* @param {number} aa\n* @returns {number}\n*/\nfunction trace_rays(width, height, aa) {\n    var ret = _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"trace_rays\"](width, height, aa);\n    return ret;\n}\n\nfunction handleError(f) {\n    return function () {\n        try {\n            return f.apply(this, arguments);\n\n        } catch (e) {\n            _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n        }\n    };\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbg_getRandomValues_57e4008f45f0e105 = handleError(function(arg0, arg1) {\n    getObject(arg0).getRandomValues(getObject(arg1));\n});\n\nconst __wbg_randomFillSync_d90848a552cbd666 = handleError(function(arg0, arg1, arg2) {\n    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));\n});\n\nconst __wbg_static_accessor_MODULE_39947eb3fe77895f = function() {\n    var ret = module;\n    return addHeapObject(ret);\n};\n\nconst __wbg_self_f865985e662246aa = handleError(function() {\n    var ret = self.self;\n    return addHeapObject(ret);\n});\n\nconst __wbg_require_c59851dfa0dc7e78 = handleError(function(arg0, arg1, arg2) {\n    var ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));\n    return addHeapObject(ret);\n});\n\nconst __wbg_crypto_bfb05100db79193b = function(arg0) {\n    var ret = getObject(arg0).crypto;\n    return addHeapObject(ret);\n};\n\nconst __wbg_msCrypto_f6dddc6ae048b7e2 = function(arg0) {\n    var ret = getObject(arg0).msCrypto;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_is_undefined = function(arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nconst __wbg_buffer_ebc6c8e75510eae3 = function(arg0) {\n    var ret = getObject(arg0).buffer;\n    return addHeapObject(ret);\n};\n\nconst __wbg_new_135e963dedf67b22 = function(arg0) {\n    var ret = new Uint8Array(getObject(arg0));\n    return addHeapObject(ret);\n};\n\nconst __wbg_set_4a5072a31008e0cb = function(arg0, arg1, arg2) {\n    getObject(arg0).set(getObject(arg1), arg2 >>> 0);\n};\n\nconst __wbg_length_317f0dd77f7a6673 = function(arg0) {\n    var ret = getObject(arg0).length;\n    return ret;\n};\n\nconst __wbg_newwithlength_78dc302d31527318 = function(arg0) {\n    var ret = new Uint8Array(arg0 >>> 0);\n    return addHeapObject(ret);\n};\n\nconst __wbg_subarray_34c228a45c72d146 = function(arg0, arg1, arg2) {\n    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nconst __wbindgen_memory = function() {\n    var ret = _wasm_ray_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"];\n    return addHeapObject(ret);\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/wasm_ray_bg.js?");

/***/ }),

/***/ "../pkg/wasm_ray_bg.wasm":
/*!*******************************!*\
  !*** ../pkg/wasm_ray_bg.wasm ***!
  \*******************************/
/*! exports provided: memory, trace_rays, __wbindgen_exn_store */
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
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm_ray_wasm_ray_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm-ray/wasm_ray_bg */ \"../pkg/wasm_ray_bg.wasm\");\n/* harmony import */ var wasm_ray__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! wasm-ray */ \"../pkg/wasm_ray.js\");\n\n\n\nconst canvas = document.getElementById(\"rendered-image\"); // as HTMLCanvasElement;\nconst render_time_label = document.getElementById(\"render-time\");\nconst slider_width = document.getElementById(\"image-width\");\nconst slider_height = document.getElementById(\"image-height\");\nconst slider_aa = document.getElementById(\"anti-aliasing\");\n\nfunction drawImage() {\n    let imWidth = Number(slider_width.value);\n    let imHeight = Number(slider_height.value);\n    let aa = Number(slider_aa.value);\n\n    slider_width.previousSibling.textContent = \"Width: \" + imWidth;\n    slider_height.previousSibling.textContent = \"Height: \" + imHeight;\n    slider_aa.previousSibling.textContent = \"Anti-Aliasing: \" + aa;\n\n    canvas.height = imHeight;\n    canvas.width = imWidth;\n\n    const t0 = performance.now();\n    const im_ptr = Object(wasm_ray__WEBPACK_IMPORTED_MODULE_1__[\"trace_rays\"])(imWidth, imHeight, aa);\n    let im = new ImageData(new Uint8ClampedArray(wasm_ray_wasm_ray_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);\n    const t1 = performance.now();\n\n    const renderTime = Math.round(t1 - t0);\n    render_time_label.textContent = \"Render Time: \" + renderTime + \"ms\";\n\n    var ctx = canvas.getContext(\"2d\");\n    ctx.putImageData(im, 0, 0);\n}\n\nslider_width.oninput = drawImage;\nslider_height.oninput = drawImage;\nslider_aa.oninput = drawImage;\n\ndrawImage();\n\n/*\nslider_width.oninput = function() {\n    let imWidth = Number(this.value);\n    let imHeight = Number(slider_height.value);\n    let aa = Number(slider_aa.value);\n\n    // TODO: abstract this all to a separate function\n    canvas.height = imHeight;\n    canvas.width = imWidth;\n    const im_ptr = trace_rays(imWidth, imHeight, aa);\n\n    let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);\n    ctx.putImageData(im, 0, 0);\n}\n\nslider_height.oninput = function() {\n    let imHeight = Number(this.value);\n    let imWidth = Number(slider_width.value);\n    let aa = Number(slider_aa.value);\n\n    canvas.height = imHeight;\n    canvas.width = imWidth;\n    const im_ptr = trace_rays(imWidth, imHeight, aa);\n\n    let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);\n    ctx.putImageData(im, 0, 0);\n}\n\nslider_aa.oninput = function() {\n    let imHeight = Number(slider_height.value);\n    let imWidth = Number(slider_width.value);\n    let aa = Number(this.value);\n\n    canvas.height = imHeight;\n    canvas.width = imWidth;\n    const im_ptr = trace_rays(imWidth, imHeight, aa);\n\n    let im = new ImageData(new Uint8ClampedArray(memory.buffer, im_ptr, 4 * imWidth * imHeight), imWidth, imHeight);\n    ctx.putImageData(im, 0, 0);\n}\n*/\n\n//# sourceURL=webpack:///./index.js?");

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