import * as echarts from 'echarts/core';
import { LineChart } from 'echarts/charts';
import {
    TitleComponent,
    TooltipComponent,
    GridComponent,
    DatasetComponent,
    TransformComponent,
} from 'echarts/components';
import { LabelLayout, UniversalTransition } from 'echarts/features';
import { CanvasRenderer } from 'echarts/renderers';
import Papa from 'papaparse';

echarts.use([
    TitleComponent,
    TooltipComponent,
    GridComponent,
    DatasetComponent,
    TransformComponent,
    LineChart,
    LabelLayout,
    UniversalTransition,
    CanvasRenderer
]);

function createTab(dom: HTMLDivElement, heads: Array<string>) {
    heads.forEach((item, index) => {
        if (index !== 0) {
            var tabItem = document.createElement('a');
            tabItem.className = 'tab-item';
            tabItem.innerText = item;
            tabItem.href = `#/${index}`;
            dom.appendChild(tabItem);
        }
    })
}

class Chart {
    private option!: echarts.EChartsCoreOption;
    private chart!: echarts.ECharts;
    private data!: Array<Array<string | number>>;
    private heads!: Array<string>;

    constructor(chartDom: HTMLDivElement, tabDom: HTMLDivElement, csv: File, head: string) {
        const complete = (result: any) => {
            const data = result.data as Array<Array<string | number>>;
            this.data = data;
            const heads = data[0] as Array<string>;
            this.heads = heads;
            createTab(tabDom, heads);

            this.update(chartDom, head);

            window.addEventListener('hashchange', () => {
                const head = location.hash.substring(2);
                this.update(chartDom, this.heads[parseInt(head)]);
            })
        }
        Papa.parse(csv, {
            dynamicTyping: true,
            escapeChar: '"',
            complete: complete
        })
    }

    update(chartDom: HTMLDivElement, head: string) {
        const i = this.heads.findIndex((item) => item === head);
        var source = this.data.map((item) => {
            return [item[0], item[i]]
        });
        this.option = {
            tooltip: {},
            xAxis: {
                type: 'value',
                axisTick: {
                    alignWithLabel: true
                },
                minorTick: {
                    show: true,
                },
                minorSplitLine: {
                    show: true,
                },
                name: (this.data[0] as Array<string>)[0],
            },
            yAxis: {
                type: 'value',
                minorTick: {
                    show: true,
                },
                minorSplitLine: {
                    show: true,
                },
                axisLabel: {
                    format: '{value}',
                },
                name: head,
                scale: true,
            },
            dataset: {
                source: source
            },
            series: [
                {
                    type: 'line',
                    showSymbol: false,
                    clip: true,
                }
            ]
        };
        this.chart = echarts.init(chartDom);
        this.chart.setOption(this.option);
        window.addEventListener('resize', () => {
            this.chart.resize();
        });
    }
}

export default Chart;