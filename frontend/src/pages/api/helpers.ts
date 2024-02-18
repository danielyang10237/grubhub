import { Group } from "./interfaces"

// returns list of all groups
export async function getAllGroups(): Promise<Group[]> {
  let response = await fetch("http://localhost:3001/groups/all");
  let value: Group[] = await response.json();
  return value;
}

// returns list of all group tags
export async function getAllTags(): Promise<string[]> {
  let response = await fetch("http://localhost:3001/tags/all");
  let value: string[] = await response.json();
  return value;
}

// given an array of tags, returns an array of groups that have at least one of the tags
export async function findGroups(searchOptions: string[]): Promise<Group[]> {
  let options = { method: "POST", body: JSON.stringify({ tags: searchOptions }), headers: { "Content-Type": "application/json" } };
  let response = await fetch("http://localhost:3001/groups/search", options);
  let value: Group[] = await response.json();
  return value;
}

// given groupID, returns group data
export async function getGroup(groupID: number): Promise<Group> {
  let response = await fetch("http://localhost:3001/groups/" + groupID.toString());
  let value: Group = await response.json()
  return value;
}