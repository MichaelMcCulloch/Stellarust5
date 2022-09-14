import React from 'react';
import './App.css';

import {
  BrowserRouter as Router, Navigate, Route, Routes, useParams
} from "react-router-dom";
function CampaignButton(props) {

  let lnk = '/campaign/' + props.campaign_name;
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
function CampaignSelect(props) {
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
      if (this.state.CampaignList) {
        return (<div><CampaignSelect data={this.state.CampaignList} /></div>)
      }

    } else {
      return (<div>404</div>)

    }

  }
}

function EmpireSelectPage() {
  let { name } = useParams();
  return (
    <div>{name}</div>
  )
}

function Index() {
  return (
    <Navigate to="/campaign_select" />
  )
}

function App() {
  return (
    <div className="App">
      <Router>
        <Routes>

          <Route path="/" element={<Index />} />
          <Route path="/campaign_select" element={<CampaignSelectPage />} />
          <Route path="/campaign/:name" element={<EmpireSelectPage />} />
        </Routes>
      </Router>

    </div >
  );
}

export default App;