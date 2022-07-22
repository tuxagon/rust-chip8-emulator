const invoke = window.__TAURI__.invoke

const WIDTH = 64
const HEIGHT = 32
const SCALE = 15
const TICKS_PER_FRAME = 10
let anim_frame = 0

const canvas = document.getElementById("canvas")
canvas.width = WIDTH * SCALE
canvas.height = HEIGHT * SCALE

const ctx = canvas.getContext("2d")
ctx.fillStyle = "black"
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)

const input = document.getElementById("fileinput")

async function mainloop() {
  for (let i = 0; i < TICKS_PER_FRAME; i++) {
    await invoke("tick")
  }

  await invoke("tick_timers")

  ctx.fillStyle = "black"
  ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)
  ctx.fillStyle = "white"
  const display = await invoke("draw_screen", { scale: SCALE })

  for (let i = 0; i < WIDTH * HEIGHT; i++) {
    if (display[i]) {
      let x = i % WIDTH;
      let y = i / WIDTH;
      
      ctx.fillRect(x * SCALE, y * SCALE, SCALE, SCALE)
    }
  }

  anim_frame = window.requestAnimationFrame(() => {
    mainloop()
  })
}

async function run() {
  document.addEventListener("keydown", function(evt) {
    console.debug(evt)
    invoke("keypress", { key: evt.key, pressed: true })
      .then((response) => console.log(response))
  })

  document.addEventListener("keyup", function(evt) {
    invoke("keypress", { key: evt.key, pressed: false })
      .then((response) => console.log(response))
  })

  input.addEventListener("change", function(evt) {
    console.debug(evt)
    // Stop previous game from rendering, if one exists
    if (anim_frame != 0) {
      window.cancelAnimationFrame(anim_frame)
    }

    let file = evt.target.files[0]
    if (!file) {
      alert("Failed to read file")
      return
    }

    // Load in game as Uint8Array
    let fr = new FileReader()
    fr.onload = function(e) {
      let buffer = fr.result
      const data = Array.from(new Uint8Array(buffer))
      console.debug(data)

      invoke("reset").then((response) => {
        console.log(response)

        invoke("load_game", { data }).then((response) => {
          console.log(response)

          mainloop()
        })
      })
    }
    fr.readAsArrayBuffer(file)
  }, false)
}

run().catch(console.error)
