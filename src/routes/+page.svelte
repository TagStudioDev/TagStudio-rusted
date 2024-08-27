<script lang="ts">
    import Alerts from '$components/Alerts.svelte';
    import Controls from '$components/Controls.svelte';
    import TSLogo from '$components/icons/TSLogo.svelte';
    // Mockup data counter
    const c = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
</script>

<div class="h-full w-full bg-base-200 px-4 pb-4">
    <div class="flex h-full w-full flex-wrap rounded-[2rem] bg-base-100 p-6">
        <div class="-mt-1 min-w-[60vw] flex-1">
            <Controls />
            <div>
                <h2 class=" text-2xl font-bold">Untagged Files</h2>
                <div class="mt-6 grid grid-cols-7 grid-rows-1">
                    <div class="card h-40 w-40 bg-base-100">
                        <figure class="px-4 py-4">
                            <div class="rounded-xl bg-base-300 p-2">
                                <TSLogo width="6rem" height="6rem" className="fill-base-100 w-full h-full" />
                            </div>
                        </figure>
                        <div class="card-body m-0 items-center p-0 text-center">
                            <h2 class=" card-title text-lg">Item</h2>
                        </div>
                    </div>
                    <div class="card h-40 w-40 bg-base-100">
                        <figure class="px-4 py-4">
                            <div class="rounded-xl bg-base-300 p-2">
                                <TSLogo width="6rem" height="6rem" className="fill-base-100 w-full h-full" />
                            </div>
                        </figure>
                        <div class="card-body m-0 items-center p-0 text-center">
                            <h2 class=" card-title text-lg">Item</h2>
                        </div>
                    </div>
                    <div class="card h-40 w-40 bg-base-100">
                        <figure class="px-4 py-4">
                            <div class="rounded-xl bg-base-300 p-2">
                                <TSLogo width="6rem" height="6rem" className="fill-base-100 w-full h-full" />
                            </div>
                        </figure>
                        <div class="card-body m-0 items-center p-0 text-center">
                            <h2 class=" card-title text-lg">Item</h2>
                        </div>
                    </div>
                </div>
            </div>
            <div class="divider"></div>
            <div class="mt-6 grid grid-cols-7">
                {#each c as i}
                    <div class="card h-40 w-40 bg-base-100">
                        <figure class="px-4 py-4">
                            <div class="rounded-xl bg-base-300 p-2">
                                <TSLogo width="6rem" height="6rem" className="fill-base-100 w-full h-full" />
                            </div>
                        </figure>
                        <div class="card-body m-0 items-center p-0 text-center">
                            <h2 class=" card-title text-lg">Item</h2>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <div class="divider divider-horizontal"></div>
        <div class=" flex-1 bg-base-100">
            <h3 class="text-2xl font-bold">Item.png</h3>
            <div class="w-fill m-4 mt-8 flex h-[13.5rem] items-center justify-center bg-base-300">
                <TSLogo width="6rem" height="6rem" className="fill-base-100 w-40 h-40" />
            </div>
        </div>
    </div>
</div>
<Alerts />
