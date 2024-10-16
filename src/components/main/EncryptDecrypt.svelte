<script lang="ts">
  import { encrypt_text, decrypt_text } from '~/tools/encrypt_decrypt';
  import { copy_text_to_clipboard } from '~/tools/kry';

  let option = $state<'encrypt' | 'decrypt'>('encrypt');
  let text = $state<string>('');
  let key = $state<string>('');
  let output = $state<string>('');
  let error_decrypting_status = $state<boolean>(false);

  async function gen_output(e: Event) {
    e.preventDefault();
    if (text === '') return;
    let error_status = false;
    if (option === 'encrypt') {
      output = await encrypt_text(text, key);
    } else if (option === 'decrypt') {
      try {
        output = await decrypt_text(text, key);
      } catch {
        output = '';
        error_status = true;
      }
    }
    error_decrypting_status = error_status;
  }
</script>

<form onsubmit={gen_output}>
  <div class="grid">
    <label>
      <input type="radio" bind:group={option} value="encrypt" /> Encrypt
    </label>
    <label>
      <input type="radio" bind:group={option} value="decrypt" /> Decrypt
    </label>
  </div>
  <label> Enter Text <textarea required bind:value={text}></textarea> </label>
  <label> Enter Key <input type="password" required bind:value={key} /> </label>
  <button type="submit">Encrypt/Decrypt Text</button>
  <label>
    Output Text
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <span onclick={() => (output = '')} class="clear_btn"></span>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <span onclick={() => copy_text_to_clipboard(output)} class="copy_btn"></span>
    {#if !error_decrypting_status}
      <textarea readonly bind:value={output}></textarea>
    {:else}
      <input type="text" aria-invalid="true" value="Wrong Key!" readonly />
    {/if}
  </label>
</form>
