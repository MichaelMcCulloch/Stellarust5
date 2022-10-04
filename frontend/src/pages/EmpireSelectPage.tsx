import React from "react";
import { useParams } from "react-router-dom";
import REMOTE_HOST from "../Const";

interface EmpireSelectPageProps {
    campaign_name: string
}
interface Empire {
    name: string
    player?: string
}
interface EmpireSelectPageState {
    empire_list?: [Empire]
}

class EmpireSelectPage extends React.Component<EmpireSelectPageProps, EmpireSelectPageState> {
    event_source?: EventSource;


    componentDidMount() {
        this.event_source = new EventSource(REMOTE_HOST + this.props.campaign_name + "/empires");
        this.event_source.onmessage = (e) => this.setState({ empire_list: JSON.parse(e.data).EmpireList })
    }
    componentWillUnmount() {
        this.event_source?.close()
    }
    render() {
        if (this.state && this.state.empire_list) return <EmpireSelectList empire_list={this.state.empire_list} campaign_name={this.props.campaign_name} />
        else return <></>
    }

}

interface EmpireButtonProps {

    campaign_name: string
    empire_name: string
    player?: string
}

function EmpireButton(props: EmpireButtonProps) {

    let lnk = '/c/' + encodeURI(props.campaign_name) + "/e/" + encodeURI(props.empire_name);

    let player;
    if (props.player) player = <div className="button_subtitle">{props.player}</div>
    return <li className="list_group_item" key={"props.key"}>
        <a className="button" href={lnk}>
            <div className="button_title">{props.empire_name}</div>
            {player}
        </a>
    </li >


}

interface EmpireSelectListProps {
    campaign_name: string
    empire_list: [Empire]
}

function EmpireSelectList(props: EmpireSelectListProps) {
    return <ul className="list_group">
        {props.empire_list.map(dict => <EmpireButton key={dict.name} empire_name={dict.name} player={dict.player} campaign_name={props.campaign_name} />)}
    </ul>;
}

const EmpireSelectPageWrapper = () => {
    const { name } = useParams();
    if (name) return <EmpireSelectPage campaign_name={name} />;
    else return <></>
};
export default EmpireSelectPageWrapper;