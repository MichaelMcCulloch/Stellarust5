import React from 'react';
import './App.css';

function CampaignButton(props) {
  return <li>
    <div>
      {props.campaign_name}
    </div>
    <div>
      {props.empire_list.map((a) => <div>{a.name} {a.player}</div>)}
    </div>
  </li>;
}
function CampaignSelect(props) {
  return <div>
    {Object.entries(props.data).map(([campaign_name, campaign_info_struct]) => <CampaignButton campaign_name={campaign_info_struct.campaign_name} empire_list={campaign_info_struct.empire_list} />)}
  </div>;
}

class CampaignSelectPage extends React.Component {

  constructor(props) {
    super(props);
    this.state = {};

  }

  componentDidMount() {
    this.eventSource = new EventSource("//localhost:8000/campaigns");

    this.eventSource.onmessage = (e) => {
      this.setState(JSON.parse(e.data));
    }
  }
  componentWillUnmount() {
    this.eventSource.close()
  }
  render() {
    if (this.state != {}) {

      return (<div><CampaignSelect data={this.state} /></div>)
    } else {
      return (<div>404</div>)

    }

  }
}


function App() {
  return (
    <div className="App">

      <CampaignSelectPage />
    </div>
  );
}

export default App;