import React from "react";
import REMOTE_HOST from "../Const";
interface CampaignState {
    campaign_list?: [CampaignInfoStruct]
};

class CampaignSelectPage extends React.Component<{}, CampaignState> {
    event_source?: EventSource;



    componentDidMount() {

        this.event_source = new EventSource(REMOTE_HOST + "campaigns");

        this.event_source.onmessage = (e) => this.setState({ campaign_list: JSON.parse(e.data).CampaignList })
    }
    componentWillUnmount() {
        this.event_source?.close()
    }
    render() {
        if (this.state && this.state.campaign_list) {
            return <CampaignSelectList campaignList={this.state.campaign_list} />
        } else {
            return <></>
        }
    }
}
interface Empire {
    name: string
    player?: string
}
interface CampaignInfoStruct {
    campaign_name: string,
    empire_list: [Empire],
};
interface CampaignButtonProps {
    campaign: CampaignInfoStruct
}
function CampaignButton(props: CampaignButtonProps) {

    let empire_select_link = '/c/' + encodeURI(props.campaign.campaign_name);
    return <li className="list_group_item" key={"props.campaign.key"}>

        <a className="button" href={empire_select_link}>
            <div className="button_title">
                {props.campaign.campaign_name}
            </div>
            <div className="button_subtitle">
                {props.campaign.empire_list.map((a) => <div key={a.name}>{a.player} | {a.name}</div>)}
            </div>
        </a>
    </li >;
}

interface CampaignListProps {
    campaignList: [CampaignInfoStruct]
}

function CampaignSelectList(props: CampaignListProps) {
    return <ul className="list_group">
        {props.campaignList.map(entry => <CampaignButton campaign={entry} />)}
    </ul>;
}

export default CampaignSelectPage;