<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Line } from 'svelte-chartjs';
	import { _ } from 'svelte-i18n'; 
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		LineElement,
		LinearScale,
		PointElement,
		TimeScale
	} from 'chart.js';
	import 'chartjs-adapter-date-fns'; 
	ChartJS.register(Title, Tooltip, Legend, LineElement, LinearScale, PointElement, TimeScale);

	let logs = [];
	let dataCalories = [];
	let dataWeight = [];

	// slider variables
	const minDays = 2;
	let maxDays = minDays;

	// default percentage
	let percentage = 70;
	$: gradient = `linear-gradient(90deg, #805AD5 ${percentage}%, #d3d3d3 ${percentage}%)`;
	$: days = maxDays <= minDays ? minDays : Math.round((percentage / 100) * (maxDays-minDays)) + minDays;

	function formatDate(dateString) {
		const options = { day: '2-digit', month: 'short' };
		const date = new Date(dateString);
		return date.toLocaleDateString(undefined, options);
	}

	onMount(async () => {
		logs = await invoke('get_all_logs');
		logs.sort((a, b) => new Date(a.entry_date) - new Date(b.entry_date));
		labels = logs.map((log) => formatDate(log.entry_date));
		
		// filter calories greater than 0 
		dataCalories = logs.map((log) => ({x: formatDate(log.entry_date), y: log.total_calories.toFixed(0)}) )
		.filter((obj) => obj.y > 0) 
		.sort((a, b) => new Date(a.x) - new Date(b.x));     
		
		// filter weight data greater than 0 
		dataWeight = logs.map((log) => ({x: formatDate(log.entry_date), y: log.weight.toFixed(2)}))
		.filter((obj) => obj.y > 0)
		.sort((a, b) => new Date(a.x) - new Date(b.x)); 
		
		maxDays = Math.max(dataWeight.length, dataCalories.length);
	});

	$: data = {
		datasets: [
			{
				label: $_('calories'),
				yAxisID: 'y1', // left y-axis
				fill: false, // disable fill under curve
				lineTension: 0.3,
				backgroundColor: 'rgba(225, 204, 230, .3)',
				borderColor: 'rgb(124, 58, 237)',
				borderCapStyle: 'butt',
				borderDash: [],
				borderDashOffset: 0.0,
				borderJoinStyle: 'miter',
				pointBorderColor: 'rgb(124, 58, 237)',
				pointBackgroundColor: 'rgb(255, 255, 255)',
				pointBorderWidth: 10,
				pointHoverRadius: 5,
				pointHoverBackgroundColor: 'rgb(0, 0, 0)',
				pointHoverBorderColor: 'rgba(220, 220, 220, 1)',
				pointHoverBorderWidth: 2,
				pointRadius: 1,
				pointHitRadius: 10,
				data: dataCalories.slice(-days)
			},
			{
				label: $_('weight'),
				yAxisID: 'y2', // right y-axis
				fill: false, // disable fill under curve
				lineTension: 0.3,
				backgroundColor: 'rgba(184, 185, 210, .3)',
				borderColor: 'rgb(6, 182, 212)',
				borderCapStyle: 'butt',
				borderDash: [],
				borderDashOffset: 0.0,
				borderJoinStyle: 'miter',
				pointBorderColor: 'rgb(6, 182, 212)',
				pointBackgroundColor: 'rgb(255, 255, 255)',
				pointBorderWidth: 10,
				pointHoverRadius: 5,
				pointHoverBackgroundColor: 'rgb(0, 0, 0)',
				pointHoverBorderColor: 'rgba(220, 220, 220, 1)',
				pointHoverBorderWidth: 2,
				pointRadius: 1,
				pointHitRadius: 10,
				data: dataWeight.slice(-days)
			}
		]
	};

	// this needs to be declared as a reactive statement in order to make it update the x-tick labels with changes in the days variable 
	$: options = {
		responsive: true,
		maintainAspectRatio: true,
		scales: {
			x: {
				type: 'time', 
				time: {
					parser: 'dd MMM', 
					tooltipFormat: 'dd MMM yyyy'
				}, 
				title: {
					display: true,
					text: $_('date')
				}, 
				ticks: {
					autoSkip: true, // auto skip labels
					maxTicksLimit: days, // limit the number of labels
				}
			},
			y1: {
				type: 'linear',
				position: 'left',
				title: {
					display: true,
					text: $_('calories')
				}
			},
			y2: {
				type: 'linear',
				position: 'right',
				title: {
					display: true, 
					text: $_('weight')
				}
			}
		}
	};
</script>

<div class="mx-4">
	<div class="text-sm tracking-tight leading-loose block">
		<span class="mr-4">
		{$_('charts.slider')} {String(days).padStart(2, '0')}
		{days === 1 ?  $_('day.singular') :  $_('day.plural') }
	</span>
		<input
			id="default-range"
			type="range"
			bind:value={percentage}
			min="1"
			max="100"
			class="w-96 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
			style="background: {gradient};"
		/>
	</div>

	<div>
		<Line {data} {options} />
	</div>
</div>

<style lang="postcss">
	input[type='range'] {
		-webkit-appearance: none;
	}

	input[type='range']::-webkit-slider-thumb {
		-webkit-appearance: none;
		border: none;
		height: 16px;
		width: 16px;
		border-radius: 50%;
		background: rgb(49, 46, 129);
		margin-top: 0px;
	}
</style>
