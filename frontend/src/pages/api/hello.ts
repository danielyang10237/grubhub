// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from "next";

import { Group, GroupSearch, GroupSearchEntry, GroupSearchOptions } from "./interfaces"

type Data = {
  name: string;
};

export async function getAllGroups(): Promise<Group[]> {
  let response = await fetch("http://localhost:3001/groups/all");
  let value: Group[] = await response.json()
  return value;
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>,
) {
  res.status(200).json({ name: "John Doe" });
}
