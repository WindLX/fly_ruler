import * as echarts from 'echarts/core';
import { LineChart } from 'echarts/charts';
import {
    TitleComponent,
    TooltipComponent,
    GridComponent,
    DatasetComponent,
    TransformComponent,
    LegendComponent,
} from 'echarts/components';
import { LabelLayout, UniversalTransition } from 'echarts/features';
import { CanvasRenderer } from 'echarts/renderers';
import Papa from 'papaparse';

echarts.use([
    LegendComponent,
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
            return [item[0], item[1], item[2], item[4]]
        });
        console.log(source)
        this.option = {
            legend: {
                // data: ['delta theta(degree)', 'delta alpha(degree)', 'delta elevator(degree)'],
                // orient: 'vertical',
                // right: 10,
                // top: 'center'
            },
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
                // name: (this.data[0] as Array<string>)[0],
                name: "planes count",
                // data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 30, 40, 50, 60, 70, 80, 90, 100]
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
                // name: head,
                // name: 'delta_theta | delta_alpha | delta_elevator (degree)',
                name: "response time(ms)",
                scale: true,

            },
            // dataset: {
            //     source: source
            // },
            series: [
                {
                    data: [[1, 0.05167], [2, 0.07637], [3, 0.05471], [4, 0.04273], [5, 0.02967],
                    [6, 0.03178], [7, 0.02197], [8, 0.02662], [9, 0.02300], [10, 0.02631],
                    [15, 0.01898], [20, 0.01484], [30, 0.01076], [40, 0.01036], [50, 0.01041],
                    [60, 0.008668], [70, 0.008760], [80, 0.007860], [90, 0.007873], [100, 0.008304]],
                    type: 'line',
                },
                // {
                //     name: 'delta theta(degree)',
                //     type: 'line',
                //     showSymbol: false,
                //     clip: true,
                //     encode: {
                //         x: 'time(s)',
                //         y: 'delta theta(degree)'
                //     }
                // },
                // {
                //     name: 'delta alpha(degree)',
                //     type: 'line',
                //     showSymbol: false,
                //     clip: true,
                //     encode: {
                //         x: 'time(s)',
                //         y: 'delta alpha(degree)'
                //     }
                // },
                // {
                //     name: 'delta elevator(degree)',
                //     type: 'line',
                //     showSymbol: false,
                //     clip: true,
                //     encode: {
                //         x: 'time(s)',
                //         y: 'delta elevator(degree)'
                //     }
                // },
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