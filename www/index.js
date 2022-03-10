//wasm-pack build in /www
//npm run start in /www
import { World } from "dodge-the-ball";
import { memory } from "dodge-the-ball/dodge_the_ball_bg"

const COLOR = "#000000"

const canvas = document.getElementById("dodge-the-ball-canvas");
const world = World.new()
const length = world.corridor_length()
const height = world.ceiling_height()

canvas.height = height
canvas.width = length

const ctx = canvas.getContext("2d")

const renderLoop = () => {
    world.tick()

    ctx.clearRect(0, 0, canvas.width, canvas.height)
    draw_world()
    draw_dodgers()
    draw_balls()

    requestAnimationFrame(renderLoop)
}

const draw_world = () => {
    ctx.beginPath()
    ctx.strokeStyle = COLOR

    ctx.moveTo(0,0)
    ctx.lineTo(length,0)
    ctx.moveTo(0,height)
    ctx.lineTo(length,height)

    ctx.stroke()
}

const draw_balls = () => {
    const ball_amount = world.get_ball_amount()
    const balls = new Float64Array(memory.buffer, world.ball_positions(), 3*ball_amount)

    ctx.beginPath()
    var x = 0.0
    for (let i = 0; i<3*ball_amount;i++) {
        if (i%3 == 2) {
            ctx.fillRect(x, balls[i], 3, 3)
        } else {
            x = balls[i]
        }
    }
}

const draw_dodgers = () => {
    const dodger_amount = world.get_dodger_amount()
    const dodgers = new Float64Array(memory.buffer, world.dodger_positions(), dodger_amount)

    ctx.beginPath()
    const x = 1000-1 // TODO: get this constant from crate::ball::configuration::CEILING_HEIGHT
    for (let i = 0; i<dodger_amount;i++) {
        ctx.fillRect(x, dodgers[i], 1, 10)
    }
}

draw_world()
draw_dodgers()
draw_balls()
requestAnimationFrame(renderLoop)
