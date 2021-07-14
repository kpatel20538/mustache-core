import { render } from "mustache-wasm";

const template = `
  <h1> Hello {{ message }}! </h1>
  <hr />
  <h5> List of Names </h5>

  <ul>
    {{# users }}
      <li> {{> user-partial }} </li>
    {{/ users }}
  </ul>
`;

const partials = {
  "user-partial": `
    <strong> {{ name }} </strong> - <em> {{ email }} </em>
  `,
};

const data = {
  message: "World",
  users: [
    { name: "Tori Asterson", email: "thetoaster45@gmail.com" },
    { name: "May Fleur", email: "mflr12@gmail.com" },
    { name: "Umi Sánchez", email: "sanchezumi@orange.co.uk" },
  ],
};

document.body.innerHTML = render(template, data, (key) => partials[key]);
