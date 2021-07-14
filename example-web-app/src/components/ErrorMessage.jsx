import React from "react";

export default function ErrorMessage({ children }) {
  return (
    <article className="message is-danger">
      <div className="message-body">{children}</div>
    </article>
  );
}
