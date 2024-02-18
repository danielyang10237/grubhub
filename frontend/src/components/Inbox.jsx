import { React } from "react";
import TopBar from "./TopBar";
import "../css/inbox.css";

const Inbox = () => {
  return (
    <div className="background">
      <TopBar />
      <div className="directory-container">
        <h1 className="inbox-title">Inbox</h1>
        <div className="mail-container">
          <div className="mails"></div>
          <div className="mail-content"></div>
        </div>
      </div>
      <img
        className="background-image-directory"
        src={`${process.env.PUBLIC_URL}/resources/coastline.jpeg`}
      ></img>
    </div>
  );
};

export default Inbox;
