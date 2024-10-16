<script lang="ts">
  import { copy_text_to_clipboard } from '~/tools/kry';
  import { to_base64, from_base64 } from '~/tools/kry';

  let option = $state<'encode' | 'decode'>('encode');
  let text = $state<string>('');
  let output = $state<string>('');
  let error_decoding_status = $state<boolean>(false);

  async function gen_output(e: Event) {
    e.preventDefault();
    if (text === '') return;
    let error_status = false;
    if (option === 'encode') {
      output = to_base64(text);
    } else if (option === 'decode') {
      try {
        output = from_base64(text);
      } catch {
        output = '';
        error_status = true;
      }
    }
    error_decoding_status = error_status;
  }
</script>

<form onsubmit={gen_output}>
  <div class="grid">
    <label>
      <input type="radio" bind:group={option} value="encode" />
      Encode
    </label>
    <label>
      <input type="radio" bind:group={option} value="decode" />
      Deocde
    </label>
  </div>
  <label>
    Enter Text
    <textarea required bind:value={text}></textarea>
  </label>
  <button type="submit">Sumbit</button>
  <label>
    Output Text
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <span onclick={() => (output = '')} class="clear_btn"></span>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <span onclick={() => copy_text_to_clipboard(output)} class="copy_btn"></span>

    {#if !error_decoding_status}
      <textarea readonly bind:value={output}></textarea>
    {:else}
      <input type="text" aria-invalid="true" value="Invalid Base64!" readonly />
    {/if}
  </label>
</form>
