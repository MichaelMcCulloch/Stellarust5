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

        }

    }
    componentWillUnmount() {

        this.eventSource.close()
    }
    render() {


        if (this.state !== {}) {
            console.log(this.state.EmpireList)
            if (this.state.EmpireList) {
                return (<div><EmpireSelectList empire_list={this.state.EmpireList} campaign_name={this.props.campaign_name} /></div>)
            }

        } else {
            return (<div>404</div>)

        }

    }
}

function EmpireButton(props) {

    let lnk = '/c/' + encodeURI(props.campaign_name) + "/e/" + encodeURI(props.empire_name);
    let player;
    if (props.player) {
        player = <div>
            {props.player}
        </div>
    } else {
        player = <></>
    }
    return <li key={"props.key"}>

        <a className="button" href={lnk}>
            <div>
                {props.empire_name}
            </div>
            {player}

        </a>
    </li >;
}
function EmpireSelectList(props) {
    return <ul>
        {props.empire_list.map(dict => <EmpireButton key={dict.name} empire_name={dict.name} player={dict.player} campaign_name={props.campaign_name} />)}
    </ul>;
}

export default EmpireSelectPageWrapper;