import './style.css'
import Chart from './chart';

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
<div id="tab"></div>
<div id="chart"></div>
`

fetch("public/data_delta.csv")
  .then(response => response.blob())
  .then(blob => {
    const csvFile = new File([blob], "data.csv", { type: "text/csv" });
    if (csvFile) {
      new Chart(
        document.getElementById('chart') as HTMLDivElement,
        document.getElementById('tab') as HTMLDivElement,
        csvFile, "delta_theta(degree)")
    }
  })
  .catch(error => console.error(error));