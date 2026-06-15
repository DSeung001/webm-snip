<script lang="ts">
  export let message: string | null = null;
</script>

{#if message}
  <section class="error-banner" role="alert">{message}</section>
{/if}
