<script lang="ts">
  import { copy_text_to_clipboard } from '~/tools/kry';
  import { scrypt } from 'hash-wasm';
  import { hashPassword } from '~/tools/better_auth_scrypt_hash';

  let text = $state<string>('');
  let hash = $state<string>('');
  let cost_factor = $state<number>(8);
  let block_size = $state<number>(8);
  let parallelism = $state<number>(1);
  let hash_length = $state<number>(64);
  let is_better_auth_hash = $state(false);

  let hashing_status = $state<boolean>(false);

  function uint8ArrayToHex(arr: Uint8Array): string {
    return Array.from(arr)
      .map((b) => b.toString(16).padStart(2, '0'))
      .join('');
  }

  async function gen_hash(e: Event) {
    e.preventDefault();
    if (text === '') return;
    const salt = crypto.getRandomValues(new Uint8Array(16));
    hashing_status = true;
    setTimeout(async () => {
      if (!is_better_auth_hash) {
        hash =
          uint8ArrayToHex(salt) +
          ':' +
          (await scrypt({
            password: text,
            salt: salt,
            costFactor: cost_factor,
            blockSize: block_size,
            parallelism: parallelism,
            hashLength: hash_length,
            outputType: 'hex'
          }));
      } else hash = await hashPassword(text);
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
    <input type="checkbox" bind:checked={is_better_auth_hash} />
    Better Auth Hash
  </label>
  <fieldset class="grid">
    <label>
      Cost Factor
      <select bind:value={cost_factor} disabled={is_better_auth_hash}>
        {#each Array.from({ length: 15 }) as _, i}
          {@const cost = Math.pow(2, i + 2)}
          <option value={cost}>{cost}</option>
        {/each}
      </select>
    </label>
    <label>
      Block Size
      <input
        type="number"
        disabled={is_better_auth_hash}
        min={1}
        max={32}
        bind:value={block_size}
      />
    </label>
  </fieldset>
  <fieldset class="grid">
    <label>
      Parallelism
      <input
        type="number"
        disabled={is_better_auth_hash}
        min={1}
        max={16}
        bind:value={parallelism}
      />
    </label>
    <label>
      Hash Length
      <input
        type="number"
        disabled={is_better_auth_hash}
        min={8}
        max={512}
        bind:value={hash_length}
      />
    </label>
  </fieldset>
  <button type="submit">Hash Text</button>
  <label>
    Hashed Scrypt Text
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
