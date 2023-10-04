<script>
  import { createEventDispatcher } from "svelte";
  import { slide } from "svelte/transition";
  const dispatch = createEventDispatcher();

  export let colors = [
    "#ff0000",
    "#00ff00",
    "#0000ff",
    "#ffff00",
    "#ff00ff",
    "#00ffff",
    "#ffffff",
    "#000000",
  ];

  function randomColor() {
    let hex = Math.floor(Math.random() * 0xffffff).toString(16);
    hex = hex.padStart(6, "0");
    return "#" + hex;
  }

  export function addColor() {
    colors = [...colors, randomColor()];
    dispatch("update");
  }

  export function removeColor() {
    colors = colors.slice(0, -1);
    dispatch("update");
  }

  export function randomizeColors() {
    colors.forEach((_, idx, arr) => {
      arr[idx] = randomColor();
    });
    colors = colors;
    dispatch("update");
  }
</script>

<div class="card flex-container">
  {#each colors as col}
    <input type="color" bind:value={col} transition:slide={{ axis: "x" }} />
  {/each}
</div>

<div class="card">
  <button on:click={addColor}>+</button>
  <button on:click={removeColor}>-</button>
  <button on:click={randomizeColors}>?</button>
</div>

<style>
  input {
    height: 2.5em;
    width: 2.5em;
    margin: 0.2em;
  }
  .flex-container {
    display: flex;
    flex-direction: row;
    justify-content: center;
    flex-wrap: wrap;
  }
</style>
