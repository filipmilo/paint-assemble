import { Canvas } from "paint-assemble";
import './index.css';

const colorButtons = document.querySelectorAll(".color-button");
const strokeButtons = document.querySelectorAll(".stroke-button");

colorButtons.forEach(button => {
  button.addEventListener("click", () => canvas.set_stroke_color(button.value));
});

strokeButtons.forEach(button => {
  button.addEventListener("click", () => canvas.set_stroke_width(button.value));
});

document.querySelector("#straight").addEventListener("click", () => canvas.setup_straight_line());

document.querySelector("#circle").addEventListener("click", () => canvas.setup_circle());

document.querySelector("#pen").addEventListener("click", () => canvas.setup_default_stroke());

const canvas = Canvas.new_canvas(800, 1500);
canvas.set_stroke_width(8)
canvas.set_stroke_color("black");
