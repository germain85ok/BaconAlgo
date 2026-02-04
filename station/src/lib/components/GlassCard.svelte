<script>
  export let className = '';
  export let hover = false;
</script>

<div class="{hover ? 'glass-card-hover' : 'glass-card'} {className}">
  <slot />
</div>

<style>
  div {
    position: relative;
  }
</style>
