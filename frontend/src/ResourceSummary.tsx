
import React from "react";
import { Chart } from "react-google-charts";

import REMOTE_HOST from "./Const";

interface ResourceSummaryChartProps {
    data: any
}

function ResourceSummaryChart(props: ResourceSummaryChartProps) {

    return <Chart
        chartType="LineChart"
        width="100%"
        height="400px"
        data={props.data}
        options={{
            title: "Alloy Stockpile Over Time",
            curveType: "function",
            legend: { position: "bottom" },
        }}
    />

};

interface ResourceSummaryProps {
    resources: string[],
    campaign_name: string,
    empire_name: string,
}

interface ResourceSummaryState {
    data: [][]
}

class ResourceSummary extends React.Component<ResourceSummaryProps, ResourceSummaryState> {
    event_source?: EventSource;
    constructor(props: ResourceSummaryProps) {
        super(props);
        this.state = { data: [] }
        this.createEventSource(this.props.resources);
    }

    shouldComponentUpdate(nextProps: ResourceSummaryProps, nextState: ResourceSummaryState) {
        if (this.props.resources !== nextProps.resources) {
            this.event_source?.close()
            nextState.data = [];
            this.createEventSource(nextProps.resources);
            return false
        } else if (this.state.data !== nextState.data && nextState.data && nextState.data[0] && nextState.data[0].length && nextState.data[0].length === nextProps.resources.length + 1) return true
        else return false
    }

    createEventSource(resources: string[]) {
        this.event_source = new EventSource(REMOTE_HOST + this.props.campaign_name + "/" + this.props.empire_name + "/resourcesummary/" + resources.join(""));
        this.event_source.onmessage = (e) => {
            const new_data = JSON.parse(e.data).ResourceSummary.map((x: [number, []]) => [x[0]].concat(x[1]));
            if (this.state) {
                this.setState({ data: this.state.data.concat(new_data) });

            } else {
                this.setState({ data: new_data });

            }
        };
    }


    componentWillUnmount() { this.event_source?.close() }

    render() {
        if (this.state && this.state.data.length > 0 && this.props.empire_name && this.props.campaign_name) {
            let data = [["Date"].concat(this.props.resources)].concat(this.state.data)
            return <ResourceSummaryChart data={data} />
        }
    }
}



export default ResourceSummary;