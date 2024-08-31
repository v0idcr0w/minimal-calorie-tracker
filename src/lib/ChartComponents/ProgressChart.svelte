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
	import { CalendarDate } from '@internationalized/date';
	import { Label } from '$lib/components/ui/label'; 
	import {formatDateYYYYMMDD} from '$lib/formatDate.js'; 
	import RangeDate from './RangeDate.svelte';

	// State variables
	let logs = [];
	let dataCalories = [];
	let dataWeight = [];
	let firstDate; 
	let lastDate; 
	let value; 

	onMount(async () => {
		// getting all logs info
		logs = await invoke('get_all_logs');
		logs.sort((a, b) => new Date(a.entry_date) - new Date(b.entry_date));
		// setting the placeholder values for the range date component
		firstDate = logs[0].entry_date;
		if (logs.length > 1) {
			lastDate = logs[logs.length - 1].entry_date;
		} else {
			lastDate = firstDate;	
		}
		const [startYear, startMonth, startDay] = firstDate.split('-');
		const [endYear, endMonth, endDay] = lastDate.split('-');

		value = {
			start: new CalendarDate(Number(startYear), Number(startMonth), Number(startDay)),
			end: new CalendarDate(Number(endYear), Number(endMonth), Number(endDay))
		};
		
		// filter calories greater than 0 
		dataCalories = logs.map((log) => ({x: log.entry_date, y: log.total_calories.toFixed(0)}) )
		.filter((obj) => obj.y > 0) 
		.sort((a, b) => new Date(a.x) - new Date(b.x));     
		
		// filter weight data greater than 0 
		dataWeight = logs.map((log) => ({x: log.entry_date, y: log.weight.toFixed(2)}))
		.filter((obj) => obj.y > 0)
		.sort((a, b) => new Date(a.x) - new Date(b.x)); 


		console.log(dataCalories); 
		
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
				data: (value && value.start && value.end) ? dataCalories.filter(point => new Date(point.x) >= value.start.toDate() && new Date(point.x) <= value.end.toDate()) : dataCalories 
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
				data: (value && value.start && value.end) ? dataWeight.filter(point => new Date(point.x) >= value.start.toDate() && new Date(point.x) <= value.end.toDate()) : dataWeight 
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
					parser: 'yyyy-MM-dd', 
					tooltipFormat: 'dd MMM yyyy'
				}, 
				title: {
					display: true,
					text: $_('date')
				}, 
				ticks: {
					autoSkip: true, // auto skip labels
					// maxTicksLimit: days, // limit the number of labels
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
	<!-- Date selector -->
	<div class="flex space-x-2 items-center pb-4">
		<Label>Select date range for plotting</Label> <RangeDate bind:value/> 
	</div>
	<!-- ! The chart -->
	<div>
		<Line {data} {options} />
	</div>
</div>