
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
    componentDidMount() {
        console.log("componentDidMount");

        this.createEventSource();


    }
    componentDidUpdate(prevProps) {
        console.log("componentDidUpdate");
        if (this.props.resources != prevProps.resources) // Check if it's a new user, you can also use some unique property, like the ID  (this.props.user.id !== prevProps.user.id)
        {
            this.eventSource.close()
            this.state.data = [];

            this.createEventSource();
        }

    }

    createEventSource() {
        console.log("createEventSource");
        let resource_code_string = this.props.resources.join("");

        this.eventSource = new EventSource(GET_REMOTE_HOST(this.props.campaign_name + "/" + this.props.empire_name + "/resourcesummary/" + resource_code_string));




        this.eventSource.onmessage = (e) => {
            const new_data = JSON.parse(e.data).ResourceSummary.map(x => [x[0]].concat(x[1]));
            this.setState({ data: this.state.data.concat(new_data) });

        };
    }
    componentWillUnmount() {
        console.log("componentWillUnmount");

        this.eventSource.close()
    }
    render() {
        console.log("render");

        if (this.state.data.length > 1) {
            if (this.props.empire_name && this.props.campaign_name) {
                let data = [["Date"].concat(this.props.resources)].concat(this.state.data)
                console.log(data);
                return <ResourceSummaryChart data={data} />
            } else {
                return <></>
            }
        }
    }
}



export default ResourceSummary;