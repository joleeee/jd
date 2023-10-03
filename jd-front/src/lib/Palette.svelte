<script>
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
</script>

<div class="card">
  {#each colors as col}
    <input type="color" bind:value={col} />
  {/each}
</div>
