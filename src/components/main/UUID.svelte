<script lang="ts">
  import { copy_text_to_clipboard } from '~/tools/kry';

  let uuid = $state<string>('');

  async function gen_salt_click() {
    uuid = crypto.randomUUID();
  }
</script>

<button onclick={gen_salt_click}>Generate UUID</button>
<label>
  Generated UUID
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <span class="clear_btn" onclick={() => (uuid = '')}></span>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <span onclick={() => copy_text_to_clipboard(uuid)} class="copy_btn"></span>
  <textarea rows="1" readonly bind:value={uuid}></textarea>
</label>
