import React from "react";
import { useParams } from "react-router-dom";

const EmpireSelectPageWrapper = () => {
    const { name } = useParams();
    return <EmpireSelectPage campaign_name={name} />;
};

class EmpireSelectPage extends React.Component {

    constructor(props) {
        super(props);
        this.state = {};

    }

    componentDidMount() {

        let source = "//localhost:8000/" + this.props.campaign_name + "/empires";
        this.eventSource = new EventSource(source);

        this.eventSource.onmessage = (e) => {
            this.setState(JSON.parse(e.data));
            console.log(e);

        }

    }
    componentWillUnmount() {

        this.eventSource.close()
    }
    render() {

        console.log(this);

        if (this.state !== {}) {
            return (<div>success</div>)

        } else {
            return (<div>404</div>)

        }

    }
}

export default EmpireSelectPageWrapper;