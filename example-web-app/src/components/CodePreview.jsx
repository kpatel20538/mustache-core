import React from "react";

function PlainPreview({ value }) {
  return (
    <div className="content m-0">
      <pre className="output">{value}</pre>
    </div>
  );
}

function RichPreview({ value }) {
  return (
    <div className="content m-0">
      <div
        className="output py-4 px-5"
        dangerouslySetInnerHTML={{ __html: value }}
      />
    </div>
  );
}

export default function CodePreview({ isRichPreview, value }) {
  return isRichPreview ? (
    <RichPreview value={value} />
  ) : (
    <PlainPreview value={value} />
  );
}
