interface Group {
  id: number;
  name: string;
  email: string | null;
  url: string | null;
  description: string | null;
  profile_photo_url: string | null;
  tags: string[];
}

let response = await fetch("http://localhost:3001/groups/all");

let body = await response.arrayBuffer();
let body_str = new TextDecoder("utf-8").decode(body);
let value: Group[] = JSON.parse(body_str);

console.log(value);
