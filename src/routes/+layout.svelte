<script>
    import '../app.css';
    // Svelte Core Imports
    let { children } = $props();
    // Component Imports
    import SplashScreen from '$components/SplashScreen.svelte';
    import Navbar from '$components/Navbar.svelte';
    import Sidebar from '$components/Sidebar.svelte';

    // For animation purposes
    let loader = $state(false);
    let loaded = $state(false);

    $effect(() => {
        // Loading trigger
        loader = true;
        // THIS IS FAKE LOADING TIME, REMOVE LATER
        setTimeout(() => {
            loaded = true;
        }, 8000);
    });
</script>

{#if !loaded && loader}
    <SplashScreen {loader} />
{:else}
    <div class="flex h-screen flex-col">
        <!-- Navbar -->
        <Navbar />

        <!-- Content below the navbar -->
        <div class="flex flex-1">
            <!-- Sidebar and Main content -->
            <Sidebar>
                <!-- Main content -->

                {@render children()}
            </Sidebar>
        </div>
    </div>
{/if}
