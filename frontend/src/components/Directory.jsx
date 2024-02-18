import { React, useEffect, useState } from "react";
import axios from "axios";
import TopBar from "./TopBar";
import DropDown from "./DropDown";
import GroupsTable from "./GroupsTable";
import "../css/directory.css";

const Directory = () => {
  const [groups, setGroups] = useState([]);
  const [interests, setInterests] = useState([]);
  const [meetingDays, setMeetingDays] = useState([]);
  const [commitmentLevels, setCommitmentLevels] = useState([]);

  const formatGroups = (input_groups) => {
    let set_groups = [];
    for (let i = 0; i < input_groups.length; i++) {
      let newGroup = {};
      newGroup.groupName = input_groups[i].name;
      newGroup.interests = input_groups[i].tags;
      newGroup.meetingTime = input_groups[i].meeting_day;
      newGroup.url = input_groups[i].url;
      const commitment = input_groups[i].commitment;
      const result = Array(commitment)
        .fill(true)
        .concat(Array(3 - commitment).fill(false));
      newGroup.commitment = result;
      set_groups.push(newGroup);
    }
    setGroups(set_groups);
  };

  useEffect(() => {
    // do the APi call with 'no-cors' mode
    axios
      .get("http://localhost:3001/groups/all", {
        mode: "no-cors",
      })
      .then((response) => {
        formatGroups(response.data);
      })
      .catch((error) => {
        console.log(error);
      });
  }, []);

  const updateGroups = async (filter, dropdown) => {
    let api_package = {};

    if (dropdown === "Interests") {
      let fitlers = [];
      for (let key in filter) {
        if (filter[key]) {
          fitlers.push(key);
        }
      }
      api_package.tags = fitlers;
      api_package.meeting_day = meetingDays;
      api_package.commitment = commitmentLevels;
      setInterests(fitlers);
    } else if (dropdown === "Meeting Day") {
      let fitlers = [];
      for (let key in filter) {
        if (filter[key]) {
          fitlers.push(key);
        }
      }
      api_package.tags = interests;
      api_package.meeting_day = fitlers;
      api_package.commitment = commitmentLevels;
      setMeetingDays(fitlers);
    } else if (dropdown === "Commitment Level") {
      let fitlers = [];
      for (let key in filter) {
        if (filter[key]) {
          fitlers.push(key);
        }
      }
      api_package.tags = interests;
      api_package.meeting_day = meetingDays;
      api_package.commitment = fitlers;
      setCommitmentLevels(fitlers);
    }

    axios
      .post("http://localhost:3001/groups/search", api_package)
      .then((response) => {
        formatGroups(response.data);
      });
  };

  return (
    <div className="background">
      <TopBar />
      <div className="directory-container">
        <h1 className="directory-title">Directory</h1>
        <div className="table">
          <div className="options-cat" style={{ flex: "0.2 1 0%" }}>
            <h3 style={{ border: "none" }} className="table-header">
              Filters
            </h3>
            <DropDown
              dropdown="Interests"
              options={["Engineering", "Tech", "Design", "Outdoors", "Sports"]}
              setFilter={updateGroups}
            />
            <DropDown
              dropdown="Meeting Day"
              options={[
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
                "Sunday",
              ]}
              setFilter={updateGroups}
            />
            <DropDown
              dropdown="Commitment Level"
              options={["I", "II", "III"]}
              setFilter={updateGroups}
            />
          </div>
          <div style={{ flex: "0.8 1 0%" }}>
            <GroupsTable groups={groups} />
          </div>
        </div>
      </div>
      <img
        className="background-image-directory"
        src={`${process.env.PUBLIC_URL}/resources/redwood.jpeg`}
      ></img>
    </div>
  );
};

export default Directory;
