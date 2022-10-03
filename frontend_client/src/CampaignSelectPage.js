import React from "react";
import REMOTE_HOST from "./Const";


class CampaignSelectPage extends React.Component {

    constructor(props) {
        super(props);
        this.state = {};

    }

    componentDidMount() {

        this.eventSource = new EventSource(REMOTE_HOST + "campaigns");

        this.eventSource.onmessage = (e) => this.setState(JSON.parse(e.data))
    }
    componentWillUnmount() {
        this.eventSource.close()
    }
    render() {
        if (this.state !== {}) {
            if (this.state.CampaignList) {
                return <CampaignSelectList data={this.state.CampaignList} />
            }
        } else {
            return <></>
        }
    }
}
function CampaignButton(props) {

    let empire_select_link = '/c/' + encodeURI(props.campaign_name);
    return <li className="list_group_item" key={"props.key"}>

        <a className="button" href={empire_select_link}>
            <div className="button_title">
                {props.campaign_name}
            </div>
            <div className="button_subtitle">
                {props.empire_list.map((a) => <div key={a.player}>{a.player} | {a.name}</div>)}
            </div>
        </a>
    </li >;
}
function CampaignSelectList(props) {
    return <ul className="list_group">
        {props.data.map(dict => <CampaignButton key={dict.campaign_name} campaign_name={dict.campaign_name} empire_list={dict.empire_list} />)}
    </ul>;
}

export default CampaignSelectPage;