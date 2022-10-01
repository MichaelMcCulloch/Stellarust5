
import React from "react";
import { Chart } from "react-google-charts";

import GET_REMOTE_HOST from "./Const";
function ResourceSummaryChart(props) {

    return <Chart
        chartType="LineChart"
        width="100%"
        height="400px"
        data={props.data.data}
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
        this.state = { data: [["date", "influence", "alloys",]] };

    }
    componentDidMount() {

        this.eventSource = new EventSource(GET_REMOTE_HOST(this.props.campaign_name + "/" + this.props.empire_name + "/resourcesummary/AlloysInfluence"));




        this.eventSource.onmessage = (e) => {
            const new_data = JSON.parse(e.data).ResourceSummaryTable.map(x => [x[0]].concat(x[1]));
            this.setState({ data: this.state.data.concat(new_data) })

        }


    }
    componentWillUnmount() {

        this.eventSource.close()
    }
    render() {

        if (this.state.data.length > 1) {

            if (this.props.empire_name && this.props.campaign_name) {
                return <ResourceSummaryChart data={this.state} />
            } else {
                return <>No Data</>
            }
        }
    }
}



export default ResourceSummary;