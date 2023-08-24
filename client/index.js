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

document.querySelector("#straight").addEventListener("click", () => canvas.set_straight_line());

document.querySelector("#circle").addEventListener("click", () => canvas.set_circle());

document.querySelector("#pen").addEventListener("click", () => canvas.set_default_stroke());

document.querySelector("#fill").addEventListener("click", () => {
  canvas.set_fill()
});

document.querySelector("#export").addEventListener("click", () => {
  const url = canvas.export();
  const download = document.createElement("a");
  download.download = "paint_assemble_export.png";
  download.href = url;
  download.click();
});

document.querySelector("#import").addEventListener("change", (event) => {
  const file = event.target.files[0];
  const reader = new FileReader();
  reader.readAsDataURL(file);

  reader.onload = (event) => {
    const uri = event.target.result;

    const img = new Image();
    img.onload = function() {
       canvas.import(img);
    };
    img.src = uri;
  }
});

const canvas = Canvas.new_canvas(800, 1500);
canvas.set_stroke_width(8)
canvas.set_stroke_color("black");
