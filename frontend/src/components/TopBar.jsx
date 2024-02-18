import React from "react";
import { Link } from "react-router-dom";
import "../css/topbar.css";

export default function TopBar() {
  return (
    <div className="menu-bar">
      <section className="menu-items">
        <div className="left-side">
          <h1>ClubHub</h1>
          <div className="vertical-sep"></div>
          <h3>Stanford</h3>
        </div>
        <div className="right-side">
          <Link className="menu-link" to="/home">
            Home
          </Link>
          <Link className="menu-link" to="/inbox">
            Inbox
          </Link>
          <Link className="menu-link" to="/directory">
            Directory
          </Link>
        </div>
      </section>
    </div>
  );
}
