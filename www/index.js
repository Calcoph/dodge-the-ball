//wasm-pack build in /www
//npm run start in /www
import { World } from "dodge-the-ball";
import { memory } from "dodge-the-ball/dodge_the_ball_bg"

const COLOR = "#000000"

const canvas = document.getElementById("dodge-the-ball-canvas");
const counters = document.getElementById("counters-div")
const world = World.new()
let tick = 0
const length = world.corridor_length()
const height = world.ceiling_height()

canvas.style.marginTop = "200px"
canvas.style.marginLeft = "400px"
canvas.height = height
canvas.width = length

const ctx = canvas.getContext("2d")

const renderLoop = () => {
    world.tick()
    console.log(tick++)

    ctx.clearRect(0, 0, canvas.width, canvas.height)
    draw_world()
    draw_dodgers()
    draw_balls()
    update_counters()

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
    const dodgers = new Float64Array(memory.buffer, world.dodger_positions(), 2*dodger_amount)

    ctx.beginPath()
    const x = world.corridor_length()-1
    var height = 0
    for (let i = 0; i<2*dodger_amount;i++) {
        if (i%2 == 0) {
            height = dodgers[i]
        }
        else {
            ctx.fillRect(x, dodgers[i], 1, height)
        }
    }
}

const init_world = () => {
    const balls = [
        {
            y_pos: 50,
            speed: 10,
            angle: 45
        },
        {
            y_pos: 50,
            speed: 10,
            angle: 60
        },
        {
            y_pos: 50,
            speed: 10,
            angle: 80
        },
        {
            y_pos: 50,
            speed: 10,
            angle: 30
        },
        {
            y_pos: 50,
            speed: 10,
            angle: 15
        },
    ]

    const dodgers = [
        {
            y_pos: 150,
            height: 5,
            max_speed: 1
        },
        {
            y_pos: 150,
            height: 150,
            max_speed: -1
        },
    ]

    for (let i = 0;i<balls.length;i++) {
        const b = balls[i]
        world.add_ball(b.y_pos, b.speed, b.angle)
    }
    
    for (let i = 0;i<dodgers.length;i++) {
        const d = dodgers[i]
        world.add_dodger(d.y_pos, d.height, d.max_speed)
        const node = document.createElement("li")
        node.appendChild(document.createTextNode(`dodger${i}: `))
        node.appendChild(document.createTextNode("0"))
        counters.appendChild(node)
    }
}

const update_counters = () => {
    const dodger_amount = world.get_dodger_amount()
    const array_start = world.get_counters()
    const counter_values = new Uint32Array(memory.buffer, array_start, dodger_amount)
    const counter_children = counters.childNodes

    for (let i = 0;i<dodger_amount;i++) {
        counter_children[i].removeChild(counter_children[i].lastChild)
        counter_children[i].appendChild(document.createTextNode(counter_values[i]))
    }

    console.log(counter_values)
}

init_world()

draw_world()
draw_dodgers()
draw_balls()
update_counters()
requestAnimationFrame(renderLoop)
