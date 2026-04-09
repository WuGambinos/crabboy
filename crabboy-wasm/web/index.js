import init,  {WebGameBoy}  from './crabboy_wasm.js';

const MAX_FPS = 60;


let lastFrameTime = performance.now();
let frameCount = 0;
let fps = 0;

function updateFrameCounter() {
    const currentTime = performance.now();
    frameCount++;

    if (currentTime - lastFrameTime >= 1000) {
        fps = frameCount;
        frameCount = 0;
        lastFrameTime = currentTime;
        let f = document.getElementById("fps");
        f.textContent = `FPS: ${fps}`
    }
}

let controllerIndex = null;
window.addEventListener("gamepadconnected", (e) => {
    const gamepad = e.gamepad;
    controllerIndex = gamepad.index;
    console.log("CONNECTED " + gamepad);
});

window.addEventListener("gamepaddisconnected", (e) => {
    controllerIndex = null;
    console.log("DISCONNECTED " + gamepad);
});


let button_pressed = null;
let button_released = null;
function handleButtons(gb, buttons) {
    for(let i = 0; i < buttons.length; i++) {
        const button = buttons[i];

        if (button.pressed) {
            console.log("BUTTON " + i);
            button_pressed = i;
                gb.on_button_down(i);
        }
        else {
            button_released = i;
            gb.on_button_up(i);
        }
    }
}

async function startWasm() {
    await init();

    let load_button = document.getElementById('load');
    load_button.addEventListener('click', () => {
        loadFile();
    });

    let resetPressed = false;
    let resetButton = document.getElementById('reset');
    resetButton.addEventListener('click', () => {
        resetPressed = true;
    });

    // Handle Keys
    let keyPressed = null;
    let keyReleased = null;
    window.addEventListener('keydown', function (e) {
        if (e.key === "Space"       || e.key === "ArrowDown"
        || e.key === "ArrowUp"      || e.key === "ArrowLeft"
        || e.key === "ArrowRight"   || e.key === "z"
        || e.key === "x") {
            e.preventDefault();
        }
        keyPressed = e.key;
    }, false);

    window.addEventListener("keyup", (e) => {
        if (e.isComposing || e.keyCode === 229) {
            return;
        }
        if (e.key === "Space"       || e.key === "ArrowDown"
        || e.key === "ArrowUp"      || e.key === "ArrowLeft"
        || e.key === "ArrowRight"   || e.key === "z"
        || e.key === "x") {
            e.preventDefault();
        }
        keyReleased = e.key;
    });


    async function loadFile() {
        const inputElement = document.getElementById('fileInput');
        const file = inputElement.files[0];

        if (!file) {
            alert('Please select a file.');
            return;
        }
        console.log("LOADING FILE");
        const fileContents = await readFileAsArrayBuffer(file);
        const contents = new Uint8Array(fileContents);

        let canvas = document.getElementById('canvas');
        let context = canvas.getContext('2d');

        let gb = new WebGameBoy();
        gb.boot(contents);
        let debug = gb.debug_info();
        let display_info = gb.display_info();
        console.log(debug);
        console.log(display_info);

        const ul = document.createElement("ul");
        document.getElementById("debug_container").appendChild(ul);

        /*
        debug.forEach(item => {
            const li = document.createElement("div");
            li.textContent = "0x" + item.toString(16).toUpperCase();
            ul.appendChild(li);
        });
        */

        let title = document.createElement("div");
        title.textContent = "TITLE: "+ inputElement.value;
        ul.appendChild(title);

        display_info.forEach(item => {
            const li = document.createElement("div");
            li.textContent = item;
            ul.appendChild(li);
        });

        game_loop();

        function game_loop() {
            updateFrameCounter();
            gb.run();
            gb.draw();

            if(controllerIndex !== null) {
                const gamepad = navigator.getGamepads()[controllerIndex];
                const press = handleButtons(gb, gamepad.buttons);
            }

            if (keyPressed != null) {
                gb.on_key_down(keyPressed);
                keyPressed = null;
            }

            if (keyReleased != null) {
                gb.on_key_up(keyReleased);
                keyReleased = null;
            }

            if (resetPressed) {
                gb.reset();
                gb.boot(contents);
                resetPressed = false;
                context.clearRect(0, 0, canvas.width, canvas.height);
            }

            requestAnimationFrame(game_loop);
        }
    }

    function readFileAsArrayBuffer(file) {
        return new Promise((resolve, reject) => {
        const reader = new FileReader();
            reader.onload = (event) => {
              resolve(event.target.result);
            };
            reader.onerror = (error) => {
          reject(error);
        };
        reader.readAsArrayBuffer(file);
      });
    };
}
startWasm();
