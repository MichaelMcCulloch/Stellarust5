import React from 'react';


import './App.css';

import {
  BrowserRouter as Router, Route, Routes
} from "react-router-dom";

import CampaignSelectPage from './CampaignSelectPage';
import EmpirePageWrapper from './EmpirePage';
import EmpireSelectPageWrapper from './EmpireSelectPage';




function App() {

  return (
    <div className="App">
      <Router>
        <Routes>

          <Route path="/" element={<CampaignSelectPage />} />
          <Route path="/c/:name" element={<EmpireSelectPageWrapper />} />
          <Route path="/c/:name/e/:empire" element={<EmpirePageWrapper />} />
        </Routes>
      </Router>

    </div >
  );
}

export default App;