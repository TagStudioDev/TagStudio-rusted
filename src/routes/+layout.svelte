<script lang="ts">
	import '../app.pcss';
	import { fade, fly } from 'svelte/transition';
	import { quartInOut, quartOut } from 'svelte/easing';

	$: outerHeight = 0;
	$: outerWidth = 0;
	$: loader = false;

	import { navigating } from '$app/stores';
	import { onMount } from 'svelte';
	let loaded = false;
	onMount(() => {
		// Loading trigger
		loader = true;
		// Loading delay in onMount, to prevent the animation from quickly starting and ending
		setTimeout(() => {
			loader = false;
		}, 7500);
		// Fake Loading Time
		setTimeout(() => {
			loaded = true;
		}, 8000);
	});
</script>

<svelte:window bind:outerWidth bind:outerHeight />

<!--Splash Screen-->
{#if !loaded}
	<div
		out:fly={{ y: -outerHeight, duration: 1200, easing: quartInOut, opacity: 1, delay: 1200 }}
		class="fixed z-50 flex h-full w-full items-center justify-center overflow-hidden bg-base-200"
	>
		<img
			id="logo-tag"
			class="tag z-[60] mr-[-1.5rem] h-[40%]"
			src="/tag-studio-tag.png"
			alt="tag"
		/>
		<img id="logo-text" class="w-[40%]" src="/tag-studio-text.png" alt="tag" />
		{#if loader}
			<svg
				in:fade={{ delay: 4500, duration: 1500 }}
				out:fade={{ duration: 1000 }}
				class="z-70 fixed translate-y-[28vh] stroke-accent"
				width="36"
				height="36"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
				><style>
					.spinner_V8m1 {
						transform-origin: center;
						animation: spinner_zKoa 2s linear infinite;
					}
					.spinner_V8m1 circle {
						stroke-linecap: round;
						animation: spinner_YpZS 1.5s ease-in-out infinite;
					}
					@keyframes spinner_zKoa {
						100% {
							transform: rotate(360deg);
						}
					}
					@keyframes spinner_YpZS {
						0% {
							stroke-dasharray: 0 150;
							stroke-dashoffset: 0;
						}
						47.5% {
							stroke-dasharray: 42 150;
							stroke-dashoffset: -16;
						}
						95%,
						100% {
							stroke-dasharray: 42 150;
							stroke-dashoffset: -59;
						}
					}
				</style><g class="spinner_V8m1"
					><circle cx="12" cy="12" r="9.5" fill="none" stroke-width="3"></circle></g
				></svg
			>
		{/if}
	</div>
{:else}
	<slot />
{/if}
