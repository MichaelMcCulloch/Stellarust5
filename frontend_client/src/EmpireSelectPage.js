import React from "react";
import { useParams } from "react-router-dom";
import GET_REMOTE_HOST from "./Const";

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
        player = <div className="button_subtitle">{props.player}</div>
    }
    return <li className="list_group_item" key={"props.key"}>
        <a className="button" href={lnk}>
            <div className="button_title">{props.empire_name}</div>
            {player}
        </a>
    </li >


}
function EmpireSelectList(props) {
    return <ul className="list_group">
        {props.empire_list.map(dict => <EmpireButton key={dict.name} empire_name={dict.name} player={dict.player} campaign_name={props.campaign_name} />)}
    </ul>;
}
const EmpireSelectPageWrapper = () => {
    const { name } = useParams();
    return <EmpireSelectPage campaign_name={name} />;
};
export default EmpireSelectPageWrapper;