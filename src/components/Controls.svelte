<script lang="ts">
    import ArrowNarrowLeft from './icons/ArrowNarrowLeft.svelte';
    import ArrowNarrowRight from './icons/ArrowNarrowRight.svelte';
    import Grid from './icons/Grid.svelte';
    import List from './icons/List.svelte';
    import RefreshCwAlt3 from './icons/RefreshCWAlt3.svelte';
    import SearchAlt2 from './icons/SearchAlt2.svelte';

    let listView = $state(true);
</script>

<div class="w-fill mb-5 flex h-10 bg-base-100">
    <button class="btn btn-ghost btn-sm m-1 px-1"><ArrowNarrowLeft className="stroke-accent w-6" /></button>
    <button class="btn btn-disabled btn-sm m-1 px-1"><ArrowNarrowRight className="stroke-accent w-6" /></button>
    <button class="btn btn-ghost btn-sm m-1 px-1"><ArrowNarrowRight className="stroke-accent w-6 -rotate-90" /></button>
    <button class="btn btn-ghost btn-sm m-1 px-1"><RefreshCwAlt3 className="stroke-accent w-6" /></button>
    <label class="input input-sm m-1 flex flex-1 items-center gap-2 bg-base-200">
        <SearchAlt2 className="stroke-accent opacity-50 w-4" />
        <input type="text" placeholder="Search Vault" class="input grow border-none p-0" />
        <kbd class="kbd kbd-sm">Ctrl</kbd>
        <kbd class="kbd kbd-sm">K</kbd>
    </label>
    <div class="join m-1 px-1">
        <button class="btn join-item {listView ? '' : 'btn-active'} btn-sm" onclick={() => (listView = false)}
            ><List className="stroke-accent w-5" /></button
        >
        <button class="btn join-item {listView ? 'btn-active' : ''} btn-sm" onclick={() => (listView = true)}
            ><Grid className="stroke-accent w-5" /></button
        >
    </div>
</div>
