<script>
	import '../app.pcss';
	import { ParaglideJS } from '@inlang/paraglide-sveltekit';
	import { i18n } from '$lib/i18n';
	import { fade, fly } from 'svelte/transition';
	import { quartInOut } from 'svelte/easing';


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

<ParaglideJS {i18n}>
	<!--Splash Screen-->
	{#if !loaded}
		<div
			out:fly={{ y: outerHeight, duration: 2000, easing: quartInOut, opacity: 1, delay: 2000 }}
			class="fixed z-50 flex h-full w-full items-center justify-center overflow-hidden bg-base-100"
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
</ParaglideJS>

<style>
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
