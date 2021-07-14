import classNames from "classnames";
import React from "react";
import { TEMPLATE } from "../state";
import Icon from "./Icon";

function Tabs({ children }) {
  return (
    <div className="tabs">
      <ul>{children}</ul>
    </div>
  );
}

function Tab({ isActive, children, onClick }) {
  return (
    <li className={classNames({ "is-active": isActive })}>
      <a
        onClick={(event) => {
          event.preventDefault();
          onClick(event);
        }}
      >
        {children}
      </a>
    </li>
  );
}

export default function TabBar({ active, tabs, onView, onRename, onAdd }) {
  return (
    <Tabs>
      <Tab
        key={TEMPLATE}
        isActive={active === TEMPLATE}
        onClick={() => onView(TEMPLATE)}
      >
        Template
      </Tab>
      {tabs.map((name) => (
        <Tab
          key={name}
          isActive={active === name}
          onClick={(event) =>
            event.detail === 2 ? onRename(name) : onView(name)
          }
        >
          {name}
        </Tab>
      ))}
      <Tab key="" onClick={onAdd}>
        <Icon name="fa-plus" />
      </Tab>
    </Tabs>
  );
}
