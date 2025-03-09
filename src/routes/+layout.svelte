<script lang="ts">
    let {children} = $props();
</script>

<nav>
    <a href="/"> Home </a>
    <a href="/settings"> Settings </a>
</nav>

{@render children()}