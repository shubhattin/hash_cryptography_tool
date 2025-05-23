<script lang="ts">
  import { scrypt } from 'hash-wasm';
  import { verifyPassword } from '~/tools/better_auth_scrypt_hash';

  let text = $state<string>('');
  let hash = $state<string>('');
  let cost_factor = $state<number>(8);
  let block_size = $state<number>(8);
  let parallelism = $state<number>(1);
  let hash_length = $state<number>(64);
  let is_better_auth_hash = $state(false);

  let verified_status = $state<boolean | null>(null);
  let hashing_status = $state<boolean>(false);

  function hexToUint8Array(hex: string): Uint8Array {
    return new Uint8Array(hex.match(/.{1,2}/g)?.map((byte) => parseInt(byte, 16)) || []);
  }

  async function gen_hash(e: Event) {
    e.preventDefault();
    if (text === '' || hash === '') return;

    hashing_status = true;
    try {
      const [storedSaltHex, storedHashHex] = hash.split(':');

      if (!storedSaltHex || !storedHashHex) {
        throw new Error('Invalid hash format');
      }

      const salt = hexToUint8Array(storedSaltHex);
      if (!is_better_auth_hash) {
        const hash_out = await scrypt({
          password: text,
          salt,
          costFactor: cost_factor,
          blockSize: block_size,
          parallelism: parallelism,
          hashLength: hash_length,
          outputType: 'hex'
        });
        verified_status = storedHashHex === hash_out;
      } else {
        verified_status = await verifyPassword({ hash: hash, password: text });
      }
    } catch (error) {
      console.error('Verification error:', error);
      verified_status = false;
    } finally {
      hashing_status = false;
    }
  }
</script>

<form onsubmit={gen_hash}>
  <label>
    Enter Text
    <textarea name="text" required bind:value={text}></textarea>
  </label>
  <label>
    Enter Hash
    <textarea name="hash" required bind:value={hash}></textarea>
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
  <button type="submit">Verify Hash</button>
  <div>
    <strong>Output :-</strong>
    {#if hashing_status}
      <span class="spinner htmx-indicator"></span>
    {/if}
    {#if verified_status}
      <input type="text" aria-invalid="false" readonly value="Valid Hash" />
    {:else if verified_status === false}
      <input type="text" aria-invalid="true" readonly value="Invalid Hash" />
    {/if}
  </div>
</form>
