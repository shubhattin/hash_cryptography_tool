<script lang="ts">
  import { copy_text_to_clipboard } from '~/tools/kry';
  import { sha256, sha512, sha3 } from 'hash-wasm';

  let hash_number = $state<'256' | '512'>('256');
  let hash_algorithm = $state<'SHA' | 'SHA3'>('SHA');
  let text = $state<string>('');
  let hash = $state<string>('');

  async function gen_hash() {
    if (hash_algorithm === 'SHA') {
      if (hash_number === '256') {
        hash = await sha256(text);
      } else {
        hash = await sha512(text);
      }
    } else if (hash_algorithm === 'SHA3') {
      if (hash_number === '256') {
        hash = await sha3(text, 256);
      } else {
        hash = await sha3(text, 512);
      }
    }
  }
</script>

<form onsubmit={gen_hash}>
  <div class="grid">
    <div class="grid">
      <label>
        <input type="radio" name="number" bind:group={hash_number} value="256" checked />
        256
      </label>
      <label>
        <input type="radio" name="number" bind:group={hash_number} value="512" />
        512
      </label>
    </div>
    <div class="grid">
      <label>
        <input type="radio" name="name" bind:group={hash_algorithm} value="SHA" checked />
        SHA
      </label>
      <label>
        <input type="radio" name="name" bind:group={hash_algorithm} value="SHA3" />
        SHA3
      </label>
    </div>
  </div>
  <small style="margin-top: 2px">
    Hash of type 256 is of length 64 and of type 512 is of length 128.
  </small>
  <label>
    Enter Text to Hash
    <textarea name="text" id="hash_input" required bind:value={text}></textarea>
  </label>
  <button type="submit">Sumbit</button>
  <label>
    Hashed Text
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <span class="clear_btn" onclick={() => (hash = '')}></span>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <span class="copy_btn" onclick={() => copy_text_to_clipboard(hash)}></span>
    <textarea readonly id="hash_out" bind:value={hash}></textarea>
  </label>
</form>
