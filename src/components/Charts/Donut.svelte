<script>
	import { Doughnut } from 'svelte-chartjs';

	const data = {
		labels: ['Documents', 'Pictures', 'Audio', 'Videos', 'Misc'],
		datasets: [
			{
				data: [4300.8, 33587.2, 723, 85913.6, 3993.6],
				backgroundColor: ['#b91c1c', '#15803d', '#1d4ed8', '#a16207', '#7e22ce']
			}
		]
	};

	import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement, CategoryScale } from 'chart.js';

	ChartJS.register(Title, Tooltip, Legend, ArcElement, CategoryScale);
</script>

<Doughnut
	class="h-full"
	{data}
	options={{
		responsive: true,
		plugins: {
			legend: {
				position: 'bottom',
				labels: {
					boxWidth: 25,
					color: '#EEEEEE',
					padding: 40
				}
			}
		}
	}}
/>
