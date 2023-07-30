import * as wasm from "paint-assemble";
import './index.css';

const colorButtons = document.querySelectorAll(".color-button");
const strokeButtons = document.querySelectorAll(".stroke-button");

colorButtons.forEach(button => {
  button.addEventListener("click", () => wasm.set_stroke_color(button.value));
});

strokeButtons.forEach(button => {
  button.addEventListener("click", () => wasm.set_stroke_width(button.value));
});

wasm.new_canvas(800, 1500);
wasm.set_stroke_width(8)
wasm.set_stroke_color("black");
