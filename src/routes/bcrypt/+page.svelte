<script>
  import BcryptHash from '~/components/main/bcrypt/BcryptHash.svelte';
  import BcryptHashVerifier from '~/components/main/bcrypt/BcryptHashVerifier.svelte';
  import ArgonHash from '~/components/main/bcrypt/ArgonHash.svelte';
  import ArgonHashVerifier from '~/components/main/bcrypt/ArgonHashVerifier.svelte';
</script>

<svelte:head>
  <title>Bcrypt</title>
</svelte:head>
<h3>Bcrypt</h3>
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
