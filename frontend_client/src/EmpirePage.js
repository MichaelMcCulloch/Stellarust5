
import React from "react";
import { useParams } from "react-router-dom";


class EmpirePage extends React.Component {

    constructor(props) {
        super(props);
        this.state = [];

    }

    render() {
        return <div>This is the page for  {this.props.empire_name} in the {this.props.campaign_name} campaign</div>
    }
}
const EmpirePageWrapper = () => {
    const { name, empire } = useParams();
    return <EmpirePage campaign_name={name} empire_name={empire} />;
};



export default EmpirePageWrapper;