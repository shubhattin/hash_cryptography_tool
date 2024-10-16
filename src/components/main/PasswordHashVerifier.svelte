<script lang="ts">
  import { sha256, sha512, sha3 } from 'hash-wasm';

  let hash_number = $state<'256' | '512'>('256');
  let hash_algorithm = $state<'SHA' | 'SHA3'>('SHA');
  let text = $state<string>('');
  let hash = $state<string>('');

  let verified_status = $state<boolean | null>(null);
  let insufficient_hash_length_status = $state<boolean>(false);

  const LENGTHS = {
    256: 64,
    512: 128
  };

  async function verify_hash(e: Event) {
    e.preventDefault();
    if (text === '') return;

    const HASH_LENGTH = LENGTHS[hash_number];
    if (hash.length < HASH_LENGTH + 1) {
      // adding 1 salt as atlleast salt of length 1 should be there
      insufficient_hash_length_status = true;
      return;
    }
    let salt_extracted = hash.substring(HASH_LENGTH);
    let hash_in = hash.substring(0, HASH_LENGTH);
    let hash_out = '';
    if (hash_algorithm === 'SHA') {
      if (hash_number === '256') {
        hash_out = await sha256(text + salt_extracted);
      } else {
        hash_out = await sha512(text + salt_extracted);
      }
    } else if (hash_algorithm === 'SHA3') {
      if (hash_number === '256') {
        hash_out = await sha3(text + salt_extracted, 256);
      } else {
        hash_out = await sha3(text + salt_extracted, 512);
      }
    }
    insufficient_hash_length_status = false;
    verified_status = hash_in === hash_out;
  }
</script>

<form onsubmit={verify_hash}>
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
  <label>
    Enter Text
    <textarea required bind:value={text}></textarea>
  </label>
  <label>
    Enter Hash
    <textarea bind:value={hash} required></textarea>
  </label>
  <button type="submit">Verify Hash</button>
  <div>
    <strong>Output :-</strong>
  </div>
  {#if insufficient_hash_length_status}
    <input type="text" aria-invalid="true" readonly value="Insufficient Hash Length" />
  {:else if verified_status}
    <input type="text" aria-invalid="false" readonly value="Valid Hash" />
  {:else if verified_status === false}
    <input type="text" aria-invalid="true" readonly value="Invalid Hash" />
  {/if}
</form>
