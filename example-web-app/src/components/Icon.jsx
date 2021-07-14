import React from "react";
import classNames from "classnames";

export default function Icon({ name, containerClassName, iconClassName }) {
  return (
    <span className={classNames("icon", containerClassName)}>
      <i className={classNames("fas", name, iconClassName)}></i>
    </span>
  );
}
