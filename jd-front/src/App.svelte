<script>
  import Palette from "./lib/Palette.svelte";
  import init, { jdither } from "./wasm/jdsite.js";
  init();

  let colors;
  $: colors, try_dither();

  let file_name = undefined;
  let src_file;

  let image_dith = undefined;
  let image_orig = undefined;

  let show_dith = true;

  $: active_image = image_dith && show_dith ? image_dith : image_orig;

  async function try_dither() {
    if (src_file && dithering_cnt === 0) {
      dither();
    }
  }

  let dithering_cnt = 0;
  async function dither() {
    dithering_cnt += 1;
    let pal = colors.map((c) => c.slice(1)).join(",");

    let bytes = new Uint8Array(await src_file.arrayBuffer());

    let result = jdither(bytes, pal);

    let reader = new FileReader();
    reader.readAsDataURL(new Blob([result.buffer]));
    reader.onloadend = function () {
      image_dith = reader.result;
    };
    dithering_cnt -= 1;
  }

  function load_file(event) {
    src_file = event.target.files[0];
    if (!src_file) {
      return;
    }
    image_dith = undefined;
    file_name = src_file.name;

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
    <input
      id="src"
      type="file"
      accept="image/png"
      on:change={load_file}
      style="display:none;"
    />
    <label class="fakeButton" for="src">{file_name || "Select Image"}</label>
  </div>

  <div class="card">
    {#if active_image}
      <div class="dither-container">
        <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
        <img
          src={active_image}
          alt="dithering display"
          on:mousedown={(event) => (event.button === 0 ? mouse_down() : null)}
          on:mouseup={mouse_up}
          on:mousemove={mouse_up}
        />
      </div>
    {:else}
      <p>No image yet!</p>
    {/if}
  </div>

  <Palette bind:colors on:update={dither} />
</main>
