<script lang="ts">
  import type { Snippet } from 'svelte';
  import '@picocss/pico';
  import '@fontsource/roboto';
  import '../app.pcss';

  let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
  <meta
    name="description"
    content="A Simple Cryptography Tool for Hashing SHA-256, SHA-512, SHA3-256 and SHA3-512, Encrypt/Decrypt Text, Base64 Encode/Decode, Salt, Password Hashing."
  />
</svelte:head>
<div class="container">
  {@render children()}
</div>
