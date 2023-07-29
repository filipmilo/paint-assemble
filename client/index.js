import * as wasm from "paint-assemble";
import './index.css';

const buttons = document.querySelectorAll(".color-button");

buttons.forEach(button => {
  button.addEventListener("click", () => wasm.set_stroke_color(button.value));
});

wasm.new_canvas(800, 1500);
wasm.set_stroke_width(8)
wasm.set_stroke_color("black");
