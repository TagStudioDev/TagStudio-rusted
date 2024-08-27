<script>
    // Component Imports
    import Spinner from '$components/icons/Spinner.svelte';
    // Component Values From Parent
    let { loader } = $props();

    // Svelte Animation Imports
    import { fly, fade } from 'svelte/transition';
    import { quartInOut } from 'svelte/easing';
</script>

<style>
    /* Logo Animation */
    #logo-tag {
        animation: logo-tag-anim cubic-bezier(0.075, 0.82, 0.165, 1) 1.5s 2.5s 1 normal forwards;
        position: absolute;
    }
    @keyframes logo-tag-anim {
        0% {
            height: 40%;
            transform: translateX(0);
        }
        30% {
            height: 25%;
            transform: translateX(calc(388px + 6vw));
        }

        100% {
            height: 25%;
            transform: translateX(calc(-388px));
        }
    }
    #logo-text {
        position: absolute;
        clip: rect(5px, 5px, 388px, 776px);
        transform: translateX(6vw);
        animation: logo-text-anim 0.87s 2.57s 1 normal forwards;
    }
    @keyframes logo-text-anim {
        30% {
            clip: rect(0px, 1552px, 388px, 776px);
        }

        100% {
            clip: rect(0px, 1552px, 388px, 0px);
        }
    }
</style>

<!-- The Splashscreen itself -->
<div
    out:fly={{ y: -outerHeight, duration: 1200, easing: quartInOut, opacity: 1, delay: 1200 }}
    class="fixed z-50 flex h-full w-full items-center justify-center overflow-hidden bg-base-200"
>
    <!-- Logo Components -->
    <img id="logo-tag" class="tag z-[60] mr-[-1.5rem] h-[40%]" src="/tag-studio-tag.png" alt="tag" />
    <img id="logo-text" class="w-[40%]" src="/tag-studio-text.png" alt="tag" />

    <!-- This key is used to trigger the loader animation on load -->
    {#key loader}
        <div in:fade|global={{ delay: 4000, duration: 1000 }} out:fade|global={{ duration: 1000 }}>
            <Spinner />
        </div>
    {/key}
</div>
