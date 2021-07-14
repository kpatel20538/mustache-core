import React from "react";

export default function FloatingActionButton({ onClick, children }) {
  return (
    <div className="fab">
      <button
        className="button is-warning is-rounded is-large"
        onClick={onClick}
      >
        {children}
      </button>
    </div>
  );
}
