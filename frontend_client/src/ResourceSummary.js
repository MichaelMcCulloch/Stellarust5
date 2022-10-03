
import React from "react";
import { Chart } from "react-google-charts";

import REMOTE_HOST from "./Const";
function ResourceSummaryChart(props) {

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

class ResourceSummary extends React.Component {

    constructor(props) {
        super(props);
        this.state = { data: [] }
        this.createEventSource(this.props.resources);
    }

    shouldComponentUpdate(nextProps, nextState) {
        if (this.props.resources !== nextProps.resources) {
            if (this.eventSource) { this.eventSource.close() }
            // We need to set this here explicitly so that when it is picked up on the next iteration of process message it is not using old data
            // eslint-disable-next-line
            this.state.data = [];
            this.createEventSource(nextProps.resources);
            return false
        } else if (this.state.data !== nextState.data && nextState.data[0].length === nextProps.resources.length + 1) return true
        else return false
    }

    createEventSource(resources) {
        this.eventSource = new EventSource(REMOTE_HOST + this.props.campaign_name + "/" + this.props.empire_name + "/resourcesummary/" + resources.join(""));
        this.eventSource.onmessage = (e) => this.processMessage(e);
    }
    processMessage(e) {
        const new_data = JSON.parse(e.data).ResourceSummary.map(x => [x[0]].concat(x[1]));
        this.setState({ data: this.state.data.concat(new_data) });
    }

    componentWillUnmount() { this.eventSource.close() }

    render() {
        if (this.state.data.length > 0 && this.props.empire_name && this.props.campaign_name) {
            let data = [["Date"].concat(this.props.resources)].concat(this.state.data)
            return <ResourceSummaryChart data={data} />
        }
    }
}



export default ResourceSummary;