<script lang="ts">
    const { className, width, height }: { className: string; width: string; height: string } = $props();
</script>

<svg {width} {height} class={className} viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M4 12H20M4 12L8 8M4 12L8 16" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
</svg>
