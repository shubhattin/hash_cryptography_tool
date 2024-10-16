<script>
  import Base64 from '~/components/main/Base64.svelte';
  import EncryptDecrypt from '~/components/main/EncryptDecrypt.svelte';
  import Hash from '~/components/main/Hash.svelte';
  import PasswordHash from '~/components/main/PasswordHash.svelte';
  import PasswordHashVerifier from '~/components/main/PasswordHashVerifier.svelte';
  import Salt from '~/components/main/Salt.svelte';
</script>

<svelte:head>
  <title>Bcrypt</title>
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
  <summary><strong>Password Hash</strong></summary>
  <PasswordHash />
</details>
<details>
  <summary><strong>Password Hash Verifier</strong></summary>
  <PasswordHashVerifier />
</details>

<a href="/bcrypt">Bcrypt</a>
