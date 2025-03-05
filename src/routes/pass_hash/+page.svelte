<script>
  import BcryptHash from '~/components/main/pass_hash/BcryptHash.svelte';
  import BcryptHashVerifier from '~/components/main/pass_hash/BcryptHashVerifier.svelte';
  import ArgonHash from '~/components/main/pass_hash/ArgonHash.svelte';
  import ArgonHashVerifier from '~/components/main/pass_hash/ArgonHashVerifier.svelte';
  import ScryptHash from '~/components/main/pass_hash/ScryptHash.svelte';
  import ScryptHashVerifier from '~/components/main/pass_hash/ScryptHashVerifier.svelte';
  import PasswordHash from '~/components/main/pass_hash/PasswordHash.svelte';
  import PasswordHashVerifier from '~/components/main/pass_hash/PasswordHashVerifier.svelte';
</script>

<svelte:head>
  <title>Password Hash</title>
</svelte:head>
<h3>Password Hash</h3>

<details>
  <summary><strong>SHA Hash</strong></summary>
  <PasswordHash />
</details>
<details>
  <summary><strong>SHA Hash Verifier</strong></summary>
  <PasswordHashVerifier />
</details>
<details>
  <summary><strong>Bcrypt Hash</strong></summary>
  <BcryptHash />
</details>
<details>
  <summary><strong>Bcrypt Hash Verifier</strong></summary>
  <BcryptHashVerifier />
</details>
<details>
  <summary><strong>Argon Hash</strong></summary>
  <ArgonHash />
</details>
<details>
  <summary><strong>Argon Hash Verifier</strong></summary>
  <ArgonHashVerifier />
</details>
<details>
  <summary><strong>Scrypt Hash</strong></summary>
  <ScryptHash />
</details>
<details>
  <summary><strong>Scrypt Hash Verifier</strong></summary>
  <ScryptHashVerifier />
</details>
