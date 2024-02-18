import { React, useState, useEffect } from "react";
import axios from "axios";
import TopBar from "./TopBar";
import "../css/inbox.css";

const Inbox = () => {
  const [mails, setMails] = useState([]);
  const [selectedMail, setSelectedMail] = useState(null);

  const setCurrentMail = (mail) => {
    setSelectedMail(mail);
  };

  useEffect(() => {
    axios
      .get("http://localhost:3001/users/0/inbox")
      .then((response) => {
        // setMails(response.data);
      })
      .catch((error) => {
        console.log(error);
      });
  }, []);

  return (
    <div className="background">
      <TopBar />
      <div className="directory-container">
        <h1 className="inbox-title">Inbox</h1>
        <div className="mail-container">
          <div className="mails">
            {mails.map((mail, index) => (
              <div key={index} className={`mail`} onClick={()=>{setCurrentMail(mail.announcement)}}>
                {mail.viewed ? null : <div className={"read-bubble"}></div>}
                <div className="mail-tag">
                  <p className="mail-sender">{mail.sender}</p>
                  <p className="mail-time">{mail.time}</p>
                  <p className="mail-title">{mail.title}</p>
                </div>
              </div>
            ))}
          </div>
          <div className="mail-content">
            {mails.map((mail, index) => (
              <div key={index} className={`mail ${mail.announcement === selectedMail} ? "" : "hidden"`}>
                <h3>{mail.title}</h3>
              </div>
            ))}
          </div>
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