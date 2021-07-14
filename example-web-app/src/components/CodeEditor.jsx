import classNames from "classnames";
import React from "react";

export default function CodeEditor({ value, onChange, className }) {
  return (
    <textarea
      className={classNames("is-family-code", className)}
      value={value}
      onChange={(event) => onChange(event.target.value)}
    />
  );
}
