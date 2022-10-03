
import React from "react";
import { useParams } from "react-router-dom";
import ResourceSummaryContainer from "./ResourceSummaryContainer";


class EmpirePage extends React.Component {

    constructor(props) {
        super(props);
        this.state = [];

    }

    render() {
        if (this.props.empire_name && this.props.campaign_name) {





            return <div><p>
                This is the page for  {this.props.empire_name} in the {this.props.campaign_name} campaign</p>
                <ResourceSummaryContainer campaign_name={this.props.campaign_name} empire_name={this.props.empire_name} />
            </div>
        } else {
            return <></>
        }
    }
}
const EmpirePageWrapper = () => {
    const { name, empire } = useParams();
    return <EmpirePage campaign_name={name} empire_name={empire} />;
};



export default EmpirePageWrapper;