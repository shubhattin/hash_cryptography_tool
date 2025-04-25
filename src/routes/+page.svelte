<script>
  import AlphaNumeric from '~/components/main/AlphaNumeric.svelte';
  import Base64 from '~/components/main/Base64.svelte';
  import EncryptDecrypt from '~/components/main/EncryptDecrypt.svelte';
  import Hash from '~/components/main/Hash.svelte';
  import Salt from '~/components/main/Salt.svelte';
  import UUID from '~/components/main/UUID.svelte';
</script>

<svelte:head>
  <title>Hash Cryptography Tool</title>
</svelte:head>

<h3>Simple Cryptography Tool</h3>
<details>
  <summary><strong>Hash</strong></summary>
  <Hash />
</details>
<details>
  <summary><strong>Encrypt/Decrypt</strong></summary>
  <EncryptDecrypt />
</details>
<details>
  <summary><strong>Base64</strong></summary>
  <Base64 />
</details>
<details>
  <summary><strong>Salt</strong></summary>
  <Salt />
</details>
<details>
  <summary><strong>UUID</strong></summary>
  <UUID />
</details>
<details>
  <summary><strong>Alphanumeric</strong></summary>
  <AlphaNumeric />
</details>

<a href="./pass_hash">Password Hash</a>
