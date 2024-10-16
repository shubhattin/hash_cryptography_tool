<script lang="ts">
  import { copy_text_to_clipboard } from '~/tools/kry';
  import { bcrypt } from 'hash-wasm';

  let text = $state<string>('');
  let hash = $state<string>('');
  let cost_factor = $state<number>(11);

  let hashing_status = $state<boolean>(false);

  async function gen_hash(e: Event) {
    e.preventDefault();
    if (text === '') return;
    const salt = crypto.getRandomValues(new Uint8Array(16));
    hashing_status = true;
    setTimeout(async () => {
      hash = await bcrypt({
        password: text,
        salt: salt,
        costFactor: cost_factor,
        outputType: 'encoded'
      });
      hashing_status = false;
    }, 0);
  }
</script>

<form onsubmit={gen_hash}>
  <label>
    Enter Text to Hash
    <textarea name="text" required bind:value={text}></textarea>
  </label>
  <label>
    Cost Factor
    <input type="number" min={6} max={20} bind:value={cost_factor} />
  </label>
  <button type="submit">Hash Text</button>
  <label>
    Hashed Bcrypt Text
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <span class="clear_btn" onclick={() => (hash = '')}></span>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <span class="copy_btn" onclick={() => copy_text_to_clipboard(hash)}></span>
    {#if hashing_status}
      <span class="spinner"></span>
    {/if}
    <textarea readonly bind:value={hash}></textarea>
  </label>
</form>
