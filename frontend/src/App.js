import "./App.css";
import Home from "./components/Home";
import Directory from "./components/Directory";
import Inbox from "./components/Inbox";
import { Routes, Route, BrowserRouter } from "react-router-dom";

function App() {
  return (
    <div className="App">
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/directory" element={<Directory />} />
          <Route path="/inbox" element={<Inbox />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
