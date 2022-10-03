import { Route, Routes } from "react-router-dom";
import './App.css';
import CampaignSelectPage from "./CampaignSelectPage";
import EmpirePageWrapper from "./EmpirePage";
import EmpireSelectPageWrapper from "./EmpireSelectPage";
export default function App() {
  return (
    <div className="App">
      <Routes>

        <Route path="/" element={<CampaignSelectPage />} />
        <Route path="/c/:name" element={<EmpireSelectPageWrapper />} />
        <Route path="/c/:name/e/:empire" element={<EmpirePageWrapper />} />
      </Routes>

    </div >
  );
}

