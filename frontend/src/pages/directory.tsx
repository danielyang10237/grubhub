import React from 'react';
import { getAllGroups } from './api/hello';
import { Group, GroupSearch, GroupSearchEntry, GroupSearchOptions } from "./api/interfaces"

// Your Directory component
const Directory = ({ groups }) => {
  return (
    <>
      <h1 className="text-4xl font-bold">This is the directory page</h1>
      <ul>
        {groups.map(group => (
          <li key={group.id}>{group.name}</li>
        ))}
      </ul>
    </>
  );
};

export default Directory;

// getServerSideProps function
export async function getServerSideProps() {
  const groups = await getAllGroups();
  return {
    props: { groups }, // will be passed to the Directory component as props
  };
}
