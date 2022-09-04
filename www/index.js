import { Simulation } from "fridge-simulator";
import { memory } from "fridge-simulator/fridge_simulator_bg";
import * as d3 from "d3";

const simulation = Simulation.new(0.016, 25.0);

console.log(simulation.rate_of_heat_gain());

const drawTemperatures = () => {
    document.getElementById('fridge_state').innerText = simulation.refrigeration_on() ? "on" : "off";

    document.getElementById('current_temp').innerText = simulation.current_temp().toFixed(2);


    const historyPtr = simulation.history();
    const history = new Float32Array(memory.buffer, historyPtr, simulation.len_history());
    // console.log(history);

    // set the dimensions and margins of the graph
    const margin = {top: 10, right: 40, bottom: 30, left: 30},
    width = 1440 + margin.left + margin.right,
    height = 400 + margin.top + margin.bottom;

    // append the svg object to the body of the page
    let svg = d3.select('svg');

    svg.attr('width', width)
        .attr('height', height);

    let scaleX = d3.scaleLinear().domain([0, 1440]).range([0, 1440]);
    let scaleY = d3.scaleLinear().domain([30, -10]).range([0, 400]);

    let axisBottom = d3.axisBottom(scaleX);
    let axisLeft = d3.axisLeft(scaleY);
    
    d3.select('#left').call(axisLeft);
    d3.select('#bottom').call(axisBottom);

    let points = svg.selectAll('circle')
        .data(history)
        .join('circle')
        .attr('cx', function(_, i) { return i+30; })
        .attr('cy', function(d, _) { return -d*10 + 300; })
        .attr('r', 1);
}

let previousTimestamp = 0;
let timeScale = 1/300;

const renderLoop = (timestamp) => {
    if (timestamp - previousTimestamp > 60000 * timeScale) {
        simulation.tick();
        drawTemperatures();
        
        previousTimestamp = timestamp;
    }

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);