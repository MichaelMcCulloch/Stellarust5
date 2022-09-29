import React from "react";
import GET_REMOTE_HOST from "./Const";
import { useParams } from "react-router-dom";

class EmpireSelectPage extends React.Component {

    constructor(props) {
        super(props);
        this.state = {};

    }

    componentDidMount() {

        this.eventSource = new EventSource(GET_REMOTE_HOST(this.props.campaign_name + "/empires"));

        this.eventSource.onmessage = (e) => this.setState(JSON.parse(e.data))

    }
    componentWillUnmount() {

        this.eventSource.close()
    }
    render() {


        if (this.state !== {}) {
            if (this.state.EmpireList) {
                return (<div><EmpireSelectList empire_list={this.state.EmpireList} campaign_name={this.props.campaign_name} /></div>)
            }

        } else {
            return (<></>)

        }

    }
}
function EmpireButton(props) {

    let lnk = '/c/' + encodeURI(props.campaign_name) + "/e/" + encodeURI(props.empire_name);

    let player;
    if (props.player) {
        player = <h3>{props.player}</h3>
    }
    return <li key={"props.key"}>
        <a className="button" href={lnk}>
            <h2>{props.empire_name}</h2>
            {player}
        </a>
    </li >


}
function EmpireSelectList(props) {
    return <ul>
        {props.empire_list.map(dict => <EmpireButton key={dict.name} empire_name={dict.name} player={dict.player} campaign_name={props.campaign_name} />)}
    </ul>;
}
const EmpireSelectPageWrapper = () => {
    const { name } = useParams();
    return <EmpireSelectPage campaign_name={name} />;
};
export default EmpireSelectPageWrapper;