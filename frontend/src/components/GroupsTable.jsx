import { React, useState, useEffect } from "react";
import "../css/groupstable.css";

const TableRow = ({ group }) => {
  const openLink = () => {
    let url = group.url;
    console.log(url);
    if (url !== "") {
      window.location.href = "https://" + url;
    }
  };

  return (
    <div className="table-header-row bottom-border not-top">
      <div className="group-column">{group.groupName}</div>
      <div className="interest-column">
        {group.interests.map((interest, index) => (
          <span key={index} className="interest-label">
            {interest} {index !== group.interests.length - 1 ? ", " : ""}
          </span>
        ))}
      </div>
      <div className="time-column">{group.meetingTime}</div>
      <div className="commitment-column">
        {group.commitment.map((level, index) => (
          <span
            key={index}
            style={{ fontWeight: "bold" }}
            className="commitment-label"
          >
            {level ? "I" : " "}
          </span>
        ))}
      </div>
      <div className="join-column">
        <button onClick={openLink} className="join-button">
          Join
        </button>
      </div>
    </div>
  );
};

const GroupsTable = (props) => {
  const [groups, setGroups] = useState([]);

  useEffect(() => {
    setGroups(props.groups);
  }, [props.groups]);

  return (
    <div className="table-container">
      <div className="table-header-row top">
        <div className="group-column">
          <p className="table-header">Groups</p>
        </div>
        <div className="interest-column">
          <p className="table-header">Interests</p>
        </div>
        <div className="time-column">
          <p className="table-header">Time</p>
        </div>
        <div className="commitment-column">
          <p className="table-header">Commitment</p>
        </div>
        <div className="join-column">
          <p className="table-header">Join!</p>
        </div>
      </div>
      {groups.map((group, index) => (
        <TableRow key={index} group={group} />
      ))}
    </div>
  );
};

export default GroupsTable;
