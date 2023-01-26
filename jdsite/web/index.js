import init, { greet, jdither } from "./wasm/jdsite.js";

async function run() {
    await init();
}

let bytes = undefined;
let img_original = undefined;
let img_dithered = undefined;

function rnd_col() {
    let hex = Math.floor(Math.random() * 0xFFFFFF).toString(16);
    hex = hex.padStart(6, '0');
    return "#" + hex;
}

window.addColor = () => {
    let list = document.getElementById("colorlist");

    let element = document.createElement("input");
    element.setAttribute("type", "color");
    element.value = rnd_col();

    let li = document.createElement("li");
    li.appendChild(element);

    list.appendChild(li);
}

window.removeColor = () => {
    let list = document.getElementById("colorlist");
    let child = list.lastElementChild;
    if (child != undefined) {
        list.removeChild(child);
    }
}

window.randomize = () => {
    let list = document.getElementById("colorlist");
    console.log(list.children);
    for (let li of list.children) {
        li.children[0].value = rnd_col();
    }

    dither();
}

window.dither = async () => {
    let pal = get_colors().join(",");
    let image = document.getElementById("dither-img");

    let result = jdither(bytes, pal);

    let reader = new FileReader();
    let blob = new Blob([result.buffer]);
    reader.readAsDataURL(blob);
    reader.onloadend = function () {
        img_dithered = reader.result;
        image.src = img_dithered;
    }
}

window.load_file = async (event) => {
    let file = event.target.files[0];
    
    img_original = URL.createObjectURL(file);
    let image = document.getElementById("dither-img");
    image.src = img_original;

    bytes = new Uint8Array(await file.arrayBuffer());

    dither();
}

window.show = () => {
    let image = document.getElementById("dither-img");
    image.src = img_dithered;
}

window.hide = () => {
    let image = document.getElementById("dither-img");
    image.src = img_original;
}

function get_colors() {
    let colors = [];

    let list = document.getElementById("colorlist");
    for (const li of list.children) {
        let hex = li.children[0].value.slice(1);
        colors.push(hex);
    }

    return colors;
}

function update_colors() {
    let list = document.getElementById("colorlist");
    list.innerHTML = "";

    for (const c of colors) {
        let li = document.createElement("li");
        li.appendChild(document.createTextNode(c));

        list.appendChild(li);
    }
}

run();
