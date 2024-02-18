/* Get information on a single group
 * Endpoint: GET /api/groups/<group_id>
 */

export interface Group {
  name: string;
  email: string | null;
  url: string | null;
  tags: string[];
}

/* Get information on a single group
 * Endpoint: GET /api/groups/search
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
