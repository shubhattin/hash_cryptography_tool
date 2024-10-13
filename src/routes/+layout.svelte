<script lang="ts">
  import type { Snippet } from 'svelte';
  import '@picocss/pico';

  let { children }: { children: Snippet } = $props();
</script>

<div class="container">
  {@render children()}
</div>
