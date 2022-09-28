import React from 'react';
import './App.css';

import {
  BrowserRouter as Router, Navigate, Route, Routes
} from "react-router-dom";

import CampaignSelectPage from './CampaignSelectPage';
import EmpireSelectPageWrapper from './EmpireSelectPage';



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
          <Route path="/campaign/:name" element={<EmpireSelectPageWrapper />} />
        </Routes>
      </Router>

    </div >
  );
}

export default App;