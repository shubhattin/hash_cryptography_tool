<script>
  import BcryptHash from '~/components/main/bcrypt/BcryptHash.svelte';
  import BcryptHashVerifier from '~/components/main/bcrypt/BcryptHashVerifier.svelte';
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
