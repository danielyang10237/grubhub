/* Get information on a single group
 * Endpoint: GET /groups/<group_id>
 */

export interface Group {
  name: string;
  email: string | null;
  url: string | null;
  description: string | null;
  profile_photo_url: string | null;
  tags: string[];
}

/* Get information on a single group
 * Endpoint: GET /groups/search
 * Parameters
 */
export interface GroupSearch {
  entries: GroupSearchEntry[];
}

export interface GroupSearchEntry {
  id: number;
  name: string;
}

export interface GroupSearchOptions {
  tags: string[];
}

export interface UserInboxResponse {
  entries: UserInboxEntry[];
}

export interface UserInboxEntry {
  group: number;
  title: string;
  body: string;
  viewed: boolean;
  announcement: number;
}
