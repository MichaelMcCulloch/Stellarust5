import React from "react";

function CampaignButton(props) {

    let lnk = '/c/' + encodeURI(props.campaign_name);
    return <li key={"props.key"}>

        <a className="button" href={lnk}>
            <div>
                {props.campaign_name}
            </div>
            <div>
                {props.empire_list.map((a) => <div key={a.player}>{a.player}</div>)}
            </div>
        </a>
    </li >;
}
function CampaignSelectList(props) {
    return <ul>
        {props.data.map(dict => <CampaignButton key={dict.campaign_name} campaign_name={dict.campaign_name} empire_list={dict.empire_list} />)}
    </ul>;
}


class CampaignSelectPage extends React.Component {

    constructor(props) {
        super(props);
        this.state = {};

    }

    componentDidMount() {

        this.eventSource = new EventSource("//127.0.0.1:8000/campaigns");

        this.eventSource.onmessage = (e) => {
            this.setState(JSON.parse(e.data));

        }
    }
    componentWillUnmount() {
        this.eventSource.close()
    }
    render() {
        if (this.state !== {}) {
            if (this.state.CampaignList) {
                return (<div><CampaignSelectList data={this.state.CampaignList} /></div>)
            }

        } else {
            return (<div>404</div>)

        }

    }
}

export default CampaignSelectPage;