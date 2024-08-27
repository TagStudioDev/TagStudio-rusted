<script lang="ts">
    import ArrowNarrowLeft from './icons/ArrowNarrowLeft.svelte';
    import ArrowNarrowRight from './icons/ArrowNarrowRight.svelte';
    import RefreshCwAlt3 from './icons/RefreshCWAlt3.svelte';
    import SearchAlt2 from './icons/SearchAlt2.svelte';
</script>

<div class="w-fill mb-5 flex h-10 bg-base-100">
    <button class="btn btn-ghost btn-sm m-1 px-1"
        ><ArrowNarrowLeft width="1.5rem" height="1.5rem" className="stroke-accent" /></button
    >
    <button class="btn btn-disabled btn-sm m-1 px-1"
        ><ArrowNarrowRight width="1.5rem" height="1.5rem" className="stroke-accent" /></button
    >
    <button class="btn btn-ghost btn-sm m-1 px-1"
        ><ArrowNarrowRight width="1.5rem" height="1.5rem" className="stroke-accent -rotate-90" /></button
    >
    <button class="btn btn-ghost btn-sm m-1 px-1"
        ><RefreshCwAlt3 width="1.5rem" height="1.5rem" className="stroke-accent" /></button
    >
    <label class="input input-sm m-1 flex flex-1 items-center gap-2 bg-base-200">
        <SearchAlt2 width="1.1rem" height="1.1rem" className="stroke-accent opacity-50" />
        <input type="text" placeholder="Search Vault" class="input grow border-none p-0" />
        <kbd class="kbd kbd-sm">Ctrl</kbd>
        <kbd class="kbd kbd-sm">K</kbd>
    </label>
</div>
