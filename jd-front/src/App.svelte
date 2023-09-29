<script>
  import init, { jdither } from "./wasm/jdsite.js";
  init();

  let src_file;

  let image_dith = undefined;
  let image_orig = undefined;

  let show_dith = true;

  $: active_image = image_dith && show_dith ? image_dith : image_orig;

  async function dither() {
    let pal = "ff0000,00ff00,0000ff,ffff00,ff00ff,00ffff,ffffff,000000";

    let bytes = new Uint8Array(await src_file.arrayBuffer());

    let result = jdither(bytes, pal);

    let reader = new FileReader();
    reader.readAsDataURL(new Blob([result.buffer]));
    reader.onloadend = function () {
      image_dith = reader.result;
    };
  }

  function load_file(event) {
    image_dith = undefined;
    src_file = event.target.files[0];

    image_orig = URL.createObjectURL(src_file);

    dither();
  }

  function mouse_down() {
    show_dith = false;
  }
  function mouse_up() {
    show_dith = true;
  }
</script>

<main>
  <h1>jd</h1>

  <div>
    <input id="src" type="file" accept="image/*" on:change={load_file} />
  </div>

  <div class="card">
    {#if active_image}
      <div class="dither-container">
        <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
        <img
          src={active_image}
          alt="dithering display"
          on:mousedown={mouse_down}
          on:mouseup={mouse_up}
        />
      </div>
    {:else}
      <p>No image yet!</p>
    {/if}
  </div>

  <div class="card">
    <!-- <button onclick="addColor()">+</button>
    <button onclick="removeColor()">-</button>
    <button onclick="randomize()">?</button>
    <button onclick="dither()">run</button> -->
  </div>
</main>
