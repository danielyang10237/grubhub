import React, { useState } from "react";
import "../css/dropdown.css"; // Ensure your CSS styles are correctly set up

const DropDown = (props) => {
  const [isOpen, setIsOpen] = useState(false);
  // Initialize checkedItems as a state to track the checked status of each option
  const [checkedItems, setCheckedItems] = useState({});

  const toggleDropdown = () => setIsOpen(!isOpen);

  const handleCheckChange = (option) => () => {
    // Update the checkedItems state in an immutable way
    const updatedCheckedItems = {
      ...checkedItems,
      [option]: !checkedItems[option],
    };

    setCheckedItems(updatedCheckedItems);
    // Call the setFilter prop with the updated checked items
    props.setFilter(updatedCheckedItems, props.dropdown);
  };

  return (
    <>
      <button onClick={toggleDropdown} className="dropdown-button">
        {props.dropdown} <span>{isOpen ? "▲" : "▼"}</span>
      </button>
      {isOpen && (
        <ul className="dropdown-menu">
          {props.options.map((option, index) => (
            <li key={index} className="dropdown-item">
              {option}
              <div
                style={{
                  width: "10px",
                  height: "10px",
                  backgroundColor: checkedItems[option] ? "red" : "transparent",
                  border: "1px solid black",
                  display: "inline-block",
                  marginLeft: "10px",
                  cursor: "pointer",
                }}
                onClick={handleCheckChange(option)} 
              ></div>
            </li>
          ))}
        </ul>
      )}
    </>
  );
};

export default DropDown;
