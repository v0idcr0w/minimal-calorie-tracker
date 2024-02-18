<script>
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { Line } from 'svelte-chartjs'; 
    import { Chart as ChartJS, Title, Tooltip, Legend, LineElement, LinearScale, PointElement, CategoryScale } from 'chart.js';
    ChartJS.register(Title, Tooltip, Legend, LineElement, LinearScale, PointElement, CategoryScale);

    let logs = []; 
    let labels = []; 
    let dataCalories = []; 
    let dataWeight = []; 
    
    // slider variables
    const minDays = 1; 
    let maxDays; 

    // default percentage 
    let percentage = 70; 
    $: gradient = `linear-gradient(90deg, #805AD5 ${percentage}%, #d3d3d3 ${percentage}%)`;
    $: days = Math.round((percentage / 100) * maxDays) || minDays; 
    
    function formatDate(dateString) {
        const options = { day: '2-digit', month: 'short'}; 
        const date = new Date(dateString); 
        return date.toLocaleDateString(undefined, options);
    }

    onMount(async () => {
        logs = await invoke('get_all_logs'); 
        logs.sort((a, b) => new Date(a.entry_date) - new Date(b.entry_date)); 
        labels = logs.map(log => formatDate(log.entry_date)); 
        maxDays = labels.length; 
        dataCalories = logs.map(log => log.total_calories.toFixed(0));
        dataWeight = logs.map(log => log.weight.toFixed(2)); 
    });

$: data = {
  labels: labels.slice(-days,),
  datasets: [
    {
      label: 'Calories',
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
      data: dataCalories.slice(-days,),
    },
    {
      label: 'Weight',
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
      data: dataWeight.slice(-days,),
    },
  ],
};

const options = {
  responsive: true, 
  scales: {
    y1: {
      type: 'linear',
      position: 'left',
    },
    y2: {
      type: 'linear',
      position: 'right',
    },
  },
};

</script>

<div class="mx-4">
  <div class="text-sm tracking-tight leading-loose flex items-center justify-between">
  
    Show the last {days} {days === 1 ? 'day' : 'days'}
  
    <input id="default-range" type="range" bind:value={percentage} min="1" max="100" class="w-4/5 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer" style="background: {gradient};">
  </div>

  <div>
    <Line {data} {options} />
  </div>
</div>

<style lang="postcss">
input[type=range]{
    -webkit-appearance: none;
}

input[type=range]::-webkit-slider-thumb {
    -webkit-appearance: none;
    border: none;
    height: 16px;
    width: 16px;
    border-radius: 50%;
    background: rgb(49, 46, 129);
    margin-top: 0px;
}

</style>