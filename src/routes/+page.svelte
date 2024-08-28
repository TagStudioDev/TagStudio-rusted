<script lang="ts">
    import Alerts from '$components/Alerts.svelte';
    import Controls from '$components/Controls.svelte';
    import DetailsViewer from '$components/DetailsViewer.svelte';
    import TSLogo from '$components/icons/TSLogo.svelte';
    import { quartInOut } from 'svelte/easing';
    import { fly } from 'svelte/transition';
    // Mockup data counter
    const c = [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24];
    const b = [1, 2, 3];
</script>

{#snippet item(i: number, d: number)}
    <div
        class="card h-40 w-40 bg-base-100"
        in:fly|global={{
            easing: quartInOut,
            duration: 500,
            opacity: 0,
            y: -25,
            delay: 100 * i + d
        }}
    >
        <figure class="px-4 py-4">
            <div class="rounded-xl bg-base-300 p-2">
                <TSLogo className="fill-base-100 w-full h-full" />
            </div>
        </figure>
        <div class="card-body m-0 items-center p-0 text-center">
            <h2 class="card-title text-base font-normal">Item</h2>
        </div>
    </div>
{/snippet}

<div class="h-full w-full bg-base-200 px-4 pb-4">
    <div class="flex h-full w-full flex-wrap rounded-[2rem] bg-base-100 p-6">
        <div class="-mt-1 min-w-[60vw] flex-1">
            <Controls />
            <div>
                <h2 class=" text-2xl font-bold">Untagged Files</h2>
                <div class="mt-6 grid grid-cols-7 grid-rows-1">
                    {#each b as i}
                        {@render item(i, 1800)}
                    {/each}
                </div>
            </div>
            <div class="divider"></div>
            <div class="mt-6 flex flex-wrap">
                {#each c as i}
                    {@render item(i, 1800)}
                {/each}
            </div>
        </div>

        <div class="divider divider-horizontal"></div>
        <DetailsViewer />
    </div>
</div>
<Alerts />
