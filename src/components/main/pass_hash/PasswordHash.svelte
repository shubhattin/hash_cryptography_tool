<script lang="ts">
  import { copy_text_to_clipboard } from '~/tools/kry';
  import { sha256, sha512, sha3 } from 'hash-wasm';
  import { gen_salt } from '~/tools/hash_tools';

  let hash_number = $state<'256' | '512'>('256');
  let hash_algorithm = $state<'SHA' | 'SHA3'>('SHA');
  let text = $state<string>('');
  let hash = $state<string>('');

  async function gen_hash(e: Event) {
    e.preventDefault();
    if (text === '') return;
    const salt = gen_salt();
    if (hash_algorithm === 'SHA') {
      if (hash_number === '256') {
        hash = (await sha256(text + salt)) + salt;
      } else {
        hash = (await sha512(text + salt)) + salt;
      }
    } else if (hash_algorithm === 'SHA3') {
      if (hash_number === '256') {
        hash = (await sha3(text + salt, 256)) + salt;
      } else {
        hash = (await sha3(text + salt, 512)) + salt;
      }
    }
  }
</script>

<small>Hashes by adding salt to text and then also appends salt into the hashed text.</small>
<form onsubmit={gen_hash}>
  <div class="grid">
    <div class="grid">
      <label>
        <input type="radio" bind:group={hash_number} value="256" checked />
        256
      </label>
      <label>
        <input type="radio" bind:group={hash_number} value="512" />
        512
      </label>
    </div>
    <div class="grid">
      <label>
        <input type="radio" bind:group={hash_algorithm} value="SHA" checked />
        SHA
      </label>
      <label>
        <input type="radio" bind:group={hash_algorithm} value="SHA3" />
        SHA3
      </label>
    </div>
  </div>
  <small style="margin-top: 2px">
    Hash of type 256 is of length 64 and of type 512 is of length 128.
  </small>
  <label>
    Enter Text to Hash
    <textarea required bind:value={text}></textarea>
  </label>
  <button type="submit">Hash Text</button>
  <label>
    Hashed Text + Salt
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <span class="clear_btn" onclick={() => (hash = '')}></span>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <span class="copy_btn" onclick={() => copy_text_to_clipboard(hash)}></span>
    <textarea readonly bind:value={hash}></textarea>
  </label>
</form>
