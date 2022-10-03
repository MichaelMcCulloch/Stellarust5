
import React from "react";
import { Chart } from "react-google-charts";

import GET_REMOTE_HOST from "./Const";
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

    }
    componentDidMount() { this.createEventSource(); }
    componentDidUpdate(prevProps) {
        if (this.props.resources != prevProps.resources) // Check if it's a new user, you can also use some unique property, like the ID  (this.props.user.id !== prevProps.user.id)
        {
            if (this.eventSource) { this.eventSource.close() }
            this.state.data = [];
            this.createEventSource();
        }
    }

    shouldComponentUpdate(nextProps, nextState) {
        if (this.props.resources !== nextProps.resources) // Check if it's a new user, you can also use some unique property, like the ID  (this.props.user.id !== prevProps.user.id)
        {

            if (this.eventSource) { this.eventSource.close() }
            this.state.data = [];
            this.eventSource = new EventSource(GET_REMOTE_HOST(this.props.campaign_name + "/" + this.props.empire_name + "/resourcesummary/" + nextProps.resources.join("")));

            this.eventSource.onmessage = (e) => {
                const new_data = JSON.parse(e.data).ResourceSummary.map(x => [x[0]].concat(x[1]));
                this.setState({ data: this.state.data.concat(new_data) });

            };
            return false
        } else if (this.state.data !== nextState.data) {
            if (nextState.data[0].length === nextProps.resources.length + 1) {

                return true

            } else {

                return false
            }
        } else {

            return false
        }
    }

    createEventSource() {


        this.eventSource = new EventSource(GET_REMOTE_HOST(this.props.campaign_name + "/" + this.props.empire_name + "/resourcesummary/" + this.props.resources.join("")));

        this.eventSource.onmessage = (e) => {
            const new_data = JSON.parse(e.data).ResourceSummary.map(x => [x[0]].concat(x[1]));
            this.setState({ data: this.state.data.concat(new_data) });

        };
    }
    componentWillUnmount() {


        this.eventSource.close()
    }
    render() {


        if (this.state.data.length > 1) {
            if (this.props.empire_name && this.props.campaign_name) {
                let data = [["Date"].concat(this.props.resources)].concat(this.state.data)
                return <ResourceSummaryChart data={data} />
            } else {
                return <></>
            }
        }
    }
}



export default ResourceSummary;