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
    let selectedAxis = 'both'; 

    function formatDate(dateString) {
        const options = { day: '2-digit', month: 'short'}; 
        const date = new Date(dateString); 
        return date.toLocaleDateString(undefined, options);
    }

    onMount(async () => {
        logs = await invoke('get_all_logs'); 
        logs.sort((a, b) => new Date(a.entry_date) - new Date(b.entry_date)); 
        labels = logs.map(log => formatDate(log.entry_date)); 
        dataCalories = logs.map(log => log.total_calories.toFixed(0));
        dataWeight = logs.map(log => log.weight.toFixed(2)); 
    });

$: data = {
  labels: labels,
  datasets: [
    {
      label: 'Calories',
      yAxisID: 'y1', // left y-axis
      fill: false, // disable fill under curve 
      lineTension: 0.3,
      backgroundColor: 'rgba(225, 204,230, .3)',
      borderColor: 'rgb(205, 130, 158)',
      borderCapStyle: 'butt',
      borderDash: [],
      borderDashOffset: 0.0,
      borderJoinStyle: 'miter',
      pointBorderColor: 'rgb(205, 130,1 58)',
      pointBackgroundColor: 'rgb(255, 255, 255)',
      pointBorderWidth: 10,
      pointHoverRadius: 5,
      pointHoverBackgroundColor: 'rgb(0, 0, 0)',
      pointHoverBorderColor: 'rgba(220, 220, 220,1)',
      pointHoverBorderWidth: 2,
      pointRadius: 1,
      pointHitRadius: 10,
      data: dataCalories,
    },
    {
      label: 'Weight',
      yAxisID: 'y2', // right y-axis
      fill: false, // disable fill under curve 
      lineTension: 0.3,
      backgroundColor: 'rgba(184, 185, 210, .3)',
      borderColor: 'rgb(35, 26, 136)',
      borderCapStyle: 'butt',
      borderDash: [],
      borderDashOffset: 0.0,
      borderJoinStyle: 'miter',
      pointBorderColor: 'rgb(35, 26, 136)',
      pointBackgroundColor: 'rgb(255, 255, 255)',
      pointBorderWidth: 10,
      pointHoverRadius: 5,
      pointHoverBackgroundColor: 'rgb(0, 0, 0)',
      pointHoverBorderColor: 'rgba(220, 220, 220, 1)',
      pointHoverBorderWidth: 2,
      pointRadius: 1,
      pointHitRadius: 10,
      data: dataWeight,
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

<!--  option... show the last x days  -->

<div>
<Line {data} {options} />
</div>