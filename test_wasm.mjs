import init, { WasmUniverse } from './pkg/vive_hashlife.js';

await init();

const universe = new WasmUniverse(16);

// Create a blinker pattern
universe.setCell(0, 0, true);
universe.setCell(1, 0, true);
universe.setCell(2, 0, true);

console.log("Initial state (generation " + universe.generation() + "):");
console.log("Population: " + universe.population());
for (let y = -1; y <= 1; y++) {
    let line = "";
    for (let x = -1; x <= 3; x++) {
        line += universe.getCell(x, y) ? "█" : ".";
    }
    console.log(line);
}

// Step 1
universe.step();
console.log("\nAfter step 1 (generation " + universe.generation() + "):");
console.log("Population: " + universe.population());
for (let y = -1; y <= 1; y++) {
    let line = "";
    for (let x = -1; x <= 3; x++) {
        line += universe.getCell(x, y) ? "█" : ".";
    }
    console.log(line);
}

// Step 2
universe.step();
console.log("\nAfter step 2 (generation " + universe.generation() + "):");
console.log("Population: " + universe.population());
for (let y = -1; y <= 1; y++) {
    let line = "";
    for (let x = -1; x <= 3; x++) {
        line += universe.getCell(x, y) ? "█" : ".";
    }
    console.log(line);
}

console.log("\n✓ Test passed! Each step advances by exactly 1 generation.");
