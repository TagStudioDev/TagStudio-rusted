<script lang="ts">
    import BoxArchive from './icons/BoxArchive.svelte';
    import CalendarDay from './icons/CalendarDay.svelte';
    import Cube from './icons/Cube.svelte';
    import LocationPin from './icons/LocationPin.svelte';
    import TSLogo from './icons/TSLogo.svelte';
</script>

<div class=" flex-1 bg-base-100">
    <h3 class="text-2xl font-bold">Item.png</h3>
    <div class="m-4 mt-8">
        <div class="w-fill flex h-[13.5rem] items-center justify-center bg-base-300">
            <TSLogo className="fill-base-100 w-40 h-40" />
        </div>
        <div class="mt-2 flex w-full flex-wrap">
            <span class="mr-5 flex"><Cube className="stroke-accent w-5 mr-2" />PNG</span>
            <span class="mr-5 flex"><BoxArchive className="stroke-accent mr-2 w-5" />456 KiB</span>
            <span class="mr-5 flex"><CalendarDay className="stroke-accent mr-2 w-5" />02/07/24</span>
            <div class="w-full">
                <span class="mr-2 mt-2 flex max-w-[] overflow-x-auto whitespace-nowrap">
                    <LocationPin className="stroke-accent w-5" />
                    <input
                        type="text"
                        readonly
                        class="input input-sm w-full max-w-xs text-base focus:border-none focus:outline-none"
                        value="/home/johndoe/Documents/School/Geography/Item.png"
                    />
                </span>
            </div>
        </div>
    </div>
</div>
