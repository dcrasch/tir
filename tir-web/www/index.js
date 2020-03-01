import * as wasm from "tir-web";

const canvas = document.getElementById('drawing');
const ctx = canvas.getContext('2d');

wasm.draw(ctx, 400, 400);