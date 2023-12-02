import * as d3 from "d3";
import data from "../rust/data.json";
import hljs from 'highlight.js/lib/core';
import javascript from 'highlight.js/lib/languages/javascript';


console.log(data);

hljs.registerLanguage('javascript', javascript);

const graphContainer = document.getElementById("graph-container");

// Specify the chart’s dimensions.
const width = 928;
const height = width;

// Create the color scale.
const color = d3.scaleLinear()
  .domain([0, 5])
  .range(["hsl(152,80%,80%)", "hsl(228,30%,40%)"])
  .interpolate(d3.interpolateHcl);

// Compute the layout.
const pack = data => d3.pack()
  .size([width, height])
  .padding(3)
  (d3.hierarchy(data.root)
    .sum(d => d.value)
    .sort((a, b) => b.value - a.value));
const root = pack(data);

// Create the SVG container.
const svg = d3.create("svg")
  .attr("viewBox", `-${width / 2} -${height / 2} ${width} ${height}`)
  .attr("width", width)
  .attr("height", height)
  .attr("style", `max-width: 100%; height: auto; display: block; margin: 0 -14px; background: ${color(0)}; cursor: pointer;`);


// Append the nodes.
const node = svg.append("g")
  .selectAll("circle")
  .data(root.descendants().slice(1))
  .join("circle")
    .attr("fill", d => d.children ? color(d.depth) : "white")
    .on("mouseover", function() { d3.select(this).attr("stroke", "#000"); })
    .on("mouseout", function() { d3.select(this).attr("stroke", null); })

// Filter for leaves, then redirect to "show connections" function
// -> "Show connections" will only trigger if we are already focused on a class
// -> "Show connections" will automatically zoom out.
// Finds the ancestory that is the decendent of the current focus.
function ancestry (d) {
  while (d.parent != focus) {
    d = d.parent;
  }
  return d;
}
node
  .filter(d => d.children === undefined || d.children.length == 0)
  .on("click", (event, d) => d.parent === focus ? 
    (showConnections(d), setInfo(d), event.stopPropagation()) :
    (zoom(event, ancestry(d)), setInfo(ancestry(d)), event.stopPropagation()));

// Filter for non-leaves, redirect to "zoom" function
node
  .filter(d => d.children !== undefined && d.children.length !== 0)
  .on("click", (event, d) => focus !== d && (zoom(event, d), setInfo(d), event.stopPropagation()));


// Append the text labels.
const label = svg.append("g")
    .style("font", "10px sans-serif")
    .attr("pointer-events", "none")
    .attr("text-anchor", "middle")
  .selectAll("text")
  .data(root.descendants())
  .join("text")
    .style("fill-opacity", d => d.parent === root ? 1 : 0)
    .style("display", d => d.parent === root ? "inline" : "none")
    .text(d => d.data.name);


// Lines
// Map edges to include node
const map = new Map(root.leaves().map(d => [d.data.name, d]));
const edges = data.edges.map(edge => {
  return {
    src: map.get(edge.src),
    dst: map.get(edge.dst)
  }
});

// Line generator
const line = d3.line()
  .x(d => d.x)
  .y(d => d.y);

// Append lines.
const lines = svg.append("g")
    .attr("stroke", "#1f1f1f")
    .attr("stroke-width", "3")
    .attr("fill", "none")
  .selectAll()
  .data(edges)
  .join("path")
    .attr("d", ({ src, dst }) => {
      const tan = (dst.y - src.y) / (dst.x - src.x);
      const ang = Math.atan(tan) + (dst.x >= src.x ? 0 : Math.PI);
      const a = {
        x: src.r * Math.cos(ang),
        y: src.r * Math.sin(ang),
      };
      const b = {
        x: dst.x - src.x - dst.r * Math.cos(ang),
        y: dst.y - src.y - dst.r * Math.sin(ang),
      };
      return line([a, b])
    })
    .style("opacity", "0")
    .style("transition", "opacity 500ms")
    .each(function(d) { d.path = this; });

// Create the zoom behavior and zoom immediately in to the initial focus node.
svg.on("click", (event) => zoom(event, root));
let focus = root;
let view;
zoomTo([focus.x, focus.y, focus.r * 2]);

function zoomTo(v) {
  const k = width / v[2];

  view = v;

  label.attr("transform", d => `translate(${(d.x - v[0]) * k},${(d.y - v[1]) * k})`);
  lines.attr("transform", ({ src }) => `
    translate(${(src.x - v[0]) * k},${(src.y - v[1]) * k})
    scale(${k})`
  );
  node.attr("transform", d => `translate(${(d.x - v[0]) * k},${(d.y - v[1]) * k})`);
  node.attr("r", d => d.r * k);
}

function zoom(event, d) {
  console.log("zoomies");
  const focus0 = focus;

  console.log(focus);
  focus = d;

  const transition = svg.transition()
      .duration(event.altKey ? 7500 : 750)
      .tween("zoom", d => {
        const i = d3.interpolateZoom(view, [focus.x, focus.y, focus.r * 2]);
        return t => zoomTo(i(t));
      });

  // Refresh label visibility
  label
    .filter(function(d) { return d.parent === focus || this.style.display === "inline"; })
    .transition(transition)
    .style("fill-opacity", d => d.parent === focus ? 1 : 0)
    .on("start", function(d) { if (d.parent === focus) this.style.display = "inline"; })
    .on("end", function(d) { if (d.parent !== focus) this.style.display = "none"; });

  // Refresh for line visibility
  lines
    .style("opacity", "0")

  return transition;
}

// -> "Show connections" will automatically zoom out.
function showConnections (fn) {
  console.log("showing connections for: ", fn);
  const transition = zoom({altKey: false}, root);

  // Show connecting lines
  const visibles = new Set();
  visibles.add(fn.data.name);
  lines
    .filter(d => d.src.data.name == fn.data.name || d.dst.data.name == fn.data.name)
    .style("opacity", "1")
    .each(d => visibles.add(d.src.data.name) && visibles.add(d.dst.data.name));

  // Show labels of connected components
  const cond = (d) => visibles.has(d.data.name) > 0;
  label
    .transition(transition)
    .style("fill-opacity", d => cond(d) ? 1 : 0)
    .on("start", function(d) { cond(d) && (this.style.display = "inline") })
    .on("end", function(d) { !cond(d) && (this.style.display = "none") });

  return;
}

// Info box setting
const nameDiv = document.getElementById("name");
const snippetDiv = document.getElementById("snippet");
function setInfo (d) {
  nameDiv.innerText = d.data.name;
  snippetDiv.innerHTML = hljs.highlight(d.data.snippet, { language: 'js' }).value;
}


// About Modal
const modal = document.getElementById("about-modal");
const aboutBtn = document.getElementById("about");
aboutBtn.onclick = () => modal.showModal();

graphContainer.appendChild(svg.node());
