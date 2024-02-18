import { React, useState, useEffect } from "react";
import axios from "axios";
import TopBar from "./TopBar";
import "../css/inbox.css";

const Inbox = () => {
  const [mails, setMails] = useState([]);
  const [selectedMail, setSelectedMail] = useState(3);

  const setCurrentMail = (mail) => {
    setSelectedMail(mail);
  };

  const getMailAPI = async () => {
    await axios
      .get("http://localhost:3001/users/1/inbox")
      .then((response) => {
        setMails(response.data);
      })
      .catch((error) => {
        console.log(error);
      });
  };

  const updateMail = (announcement, id) => {
    axios
      .post("http://localhost:3001/users/mark_as_read", {
        announcement: announcement,
        user: id,
      })
      .then((response) => {
        getMailAPI();
      })
      .catch((error) => {
        console.log(error);
      });
  };

  useEffect(() => {
    getMailAPI();
  }, []);

  return (
    <div className="background">
      <TopBar />
      <div className="directory-container">
        <h1 className="inbox-title">Inbox</h1>
        <div className="mail-container">
          <div className="mails">
            {mails.map((mail, index) => (
              <div
                key={index}
                className={`mail ${
                  mail.announcement === selectedMail ? "selected-mail" : ""
                }`}
                onClick={() => {
                  setCurrentMail(mail.announcement);
                  updateMail(mail.announcement, 1);
                }}
              >
                <div className="mail-sender">
                  {mail.viewed ? (
                    <div></div>
                  ) : (
                    <div className={"read-bubble"}></div>
                  )}
                  {mail.sender}
                </div>
                <p className="mail-time">{mail.time}</p>
              </div>
            ))}
          </div>
          <div className="mail-content">
            {mails.map((mail, index) => (
              <div
                key={index}
                className={`message ${
                  mail.announcement === selectedMail ? "" : "hidden"
                }`}
              >
                <h3 className="subject-line">Subject: {mail.subject}</h3>
                <h4 className="mail-words">{mail.body}</h4>
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
