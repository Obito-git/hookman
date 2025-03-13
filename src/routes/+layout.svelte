<script lang="ts">
    let {children} = $props();
    import "../app.css"
</script>

<nav>
    <a href="/"> Home </a>
    <a href="/settings"> Settings </a>
</nav>

{@render children()}