<script lang="ts">
  import { copy_text_to_clipboard } from '~/tools/kry';
  import { argon2id, argon2d, argon2i } from 'hash-wasm';

  let text = $state<string>('');
  let hash = $state<string>('');
  let parallelism = $state<number>(2);
  let iteration_count = $state<number>(128);
  let memory_size = $state<number>(512);
  let hash_length = $state<number>(32);
  let hash_type = $state<'argon2id' | 'argon2d' | 'argon2i'>('argon2id');

  let hashing_status = $state<boolean>(false);

  async function gen_hash(e: Event) {
    e.preventDefault();
    if (text === '') return;
    const salt = crypto.getRandomValues(new Uint8Array(16));
    hashing_status = true;
    setTimeout(async () => {
      const hashFunction = {
        argon2id: argon2id,
        argon2d: argon2d,
        argon2i: argon2i
      }[hash_type];

      hash = await hashFunction({
        password: text,
        salt,
        parallelism,
        iterations: iteration_count,
        memorySize: memory_size,
        hashLength: hash_length,
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
  <fieldset class="grid">
    <label>
      Hash Algorithm:
      <select bind:value={hash_type}>
        <option value="argon2id">Argon2id</option>
        <option value="argon2d">Argon2d</option>
        <option value="argon2i">Argon2i</option>
      </select>
    </label>
    <label>
      Parallelism:
      <input type="number" min={1} max={16} bind:value={parallelism} placeholder="1-16" />
    </label>
  </fieldset>
  <fieldset class="grid">
    <label>
      Iterations:
      <input type="number" min={2} max={65536} bind:value={iteration_count} placeholder="2-65536" />
    </label>
    <label>
      Memory Size (KB):
      <input
        type="number"
        min={64}
        max={1048576}
        bind:value={memory_size}
        placeholder="64-1048576"
      />
    </label>
    <label>
      Hash Length (bytes):
      <input type="number" min={16} max={64} bind:value={hash_length} placeholder="16-64" />
    </label>
  </fieldset>
  <button type="submit">Hash Text</button>
  <label>
    Hashed {hash_type} Text
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
