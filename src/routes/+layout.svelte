<script lang="ts">
  import type { Snippet } from 'svelte';
  import '@picocss/pico';
  import '@fontsource/roboto';
  import '../app.pcss';

  let { children }: { children: Snippet } = $props();
</script>

<div class="container">
  {@render children()}
</div>
