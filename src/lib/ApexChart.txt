<script lang="ts">
	import { onMount } from 'svelte';
	import AC from 'apexcharts';

	export let options;

	let ApexCharts: AC;
	let loaded = false;

	const chart = (node, options) => {
		if (!loaded) return;

		let myChart = new AC(node, options);
		myChart.render();

		return {
			update(options) {
				myChart.updateOptions(options);
			},
			destroy() {
				myChart.destroy();
			}
		};
	};

	onMount(async () => {
		//@ts-ignore
		window.ApexCharts = await import('apexcharts');
		loaded = true;
	});
</script>

{#if loaded}
	<div use:chart={options}></div>
{/if}
