<script lang="ts">
  import { copy_text_to_clipboard } from '~/tools/kry';

  let code = $state('');
  let length = $state(32);

  function generateRandomAlphanumeric(length: number) {
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let result = '';
    const randomBytes = new Uint8Array(length);
    crypto.getRandomValues(randomBytes);
    for (let i = 0; i < length; i++) {
      result += characters[randomBytes[i] % characters.length];
    }
    return result;
  }

  async function gen_salt_click() {
    code = generateRandomAlphanumeric(length);
  }
</script>

<div>
  <label>
    Length
    <input type="number" bind:value={length} />
  </label>
  <button onclick={gen_salt_click}>Generate Code</button>
</div>
<label>
  Generated Code
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <span class="clear_btn" onclick={() => (code = '')}></span>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <span onclick={() => copy_text_to_clipboard(code)} class="copy_btn"></span>
  <textarea rows="1" readonly bind:value={code}></textarea>
</label>
