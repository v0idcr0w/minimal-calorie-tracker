<script>
    import { Doughnut } from 'svelte-chartjs';
    import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement, CategoryScale, Filler} from 'chart.js';

    export let macros; // order: protein, carbohydrates, fats 

    $: data = {
    labels: ['Protein', 'Carbohydrates', 'Fats'],
    datasets: [
      {
        data: macros,
        backgroundColor: ['#F7464A', '#46BFBD', '#FDB45C'],
        hoverBackgroundColor: [
          '#FF5A5E',
          '#5AD3D1',
          '#FFC870',
        ],
      },
    ],
  };
    ChartJS.register(Title, Tooltip, Legend, ArcElement, CategoryScale, Filler);


    // custom tooltip to render the percent sign and correct number of decimal places 
    let options = {
    responsive: true,
    plugins: {
      legend: {
        position: 'right',
        labels: {
          boxWidth: 10, 
        },
      },
      tooltip: {
        callbacks: {
          label: function(context) {
            let label = context.label || '';
            if (label) {
              label += ': ';
            }
            if (context.raw !== undefined) {
              label += context.raw.toFixed(1) + '%';
            }
            return label;
          }
        }
      }
    }
  };

</script>

<div style="width: 225px;">
    <Doughnut {data} {options} />
</div> 