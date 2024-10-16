<script lang="ts">
  import { bcryptVerify } from 'hash-wasm';

  let hash = $state<string>('');
  let text = $state<string>('');

  let verified_status = $state<boolean | null>(null);
  let hashing_status = $state(false);
  let insufficient_hash_length_status = $state(false);

  async function verify_hash(e: Event) {
    e.preventDefault();
    hashing_status = true;
    if (text === '') return;
    if (hash.length !== 60) {
      insufficient_hash_length_status = true;
      return;
    }
    setTimeout(async () => {
      verified_status = await bcryptVerify({
        password: text,
        hash: hash
      });
      hashing_status = false;
      insufficient_hash_length_status = false;
    }, 0);
  }
</script>

<form onsubmit={verify_hash}>
  <label>
    Enter Text
    <textarea required bind:value={text}></textarea>
  </label>
  <label>
    Enter Hash
    <textarea required bind:value={hash}></textarea>
  </label>
  <button type="submit">Verify</button>
  <div>
    <strong>Output :-</strong>
    {#if hashing_status}
      <span class="spinner htmx-indicator"></span>
    {/if}
    {#if insufficient_hash_length_status}
      <input type="text" aria-invalid="true" readonly value="Insufficient Hash Length" />
    {:else if verified_status}
      <input type="text" aria-invalid="false" readonly value="Valid Hash" />
    {:else if verified_status === false}
      <input type="text" aria-invalid="true" readonly value="Invalid Hash" />
    {/if}
  </div>
</form>
