<script lang="ts">
  import { gen_salt } from '~/tools/hash_tools';
  import { copy_text_to_clipboard } from '~/tools/kry';

  let salt = $state<string>('');

  async function gen_salt_click() {
    salt = gen_salt();
  }
</script>

<small>Salt is 32 character long random value</small>
<button onclick={gen_salt_click}>Generate Salt</button>
<label>
  Generated Salt
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <span class="clear_btn" onclick={() => (salt = '')}></span>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <span onclick={() => copy_text_to_clipboard(salt)} class="copy_btn"></span>
  <textarea rows="1" readonly bind:value={salt}></textarea>
</label>
