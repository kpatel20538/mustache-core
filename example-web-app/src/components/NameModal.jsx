import React from "react";
import classNames from "classnames";
import Icon from "./Icon";

function Modal({ isActive, onCancel, children }) {
  return (
    <div className={classNames("modal", { "is-active": isActive })}>
      <div className="modal-background"></div>
      <div className="modal-content">{children}</div>
      <button
        className="modal-close is-large"
        aria-label="close"
        onClick={onCancel}
      ></button>
    </div>
  );
}

function ControlButton({ className, disabled, onClick, children }) {
  return (
    <div className="control">
      <button
        className={classNames("button", className)}
        disabled={disabled}
        onClick={onClick}
      >
        {children}
      </button>
    </div>
  );
}

function FieldInput({ label, placeholder, error, value, onChange, onSubmit }) {
  return (
    <div className="field">
      <label className="label">{label}</label>
      <div className="control">
        <input
          className={classNames("input", { "is-danger": error })}
          type="text"
          placeholder={placeholder}
          value={value}
          onChange={(event) => onChange(event.target.value)}
          onKeyUp={(event) => event.key === "Enter" && onSubmit?.()}
        />
        <p className="help is-danger">{error ?? <br />}</p>
      </div>
    </div>
  );
}

export default function NameModal({
  value,
  error,
  isActive,
  isRename,
  onChange,
  onCancel,
  onSubmit,
  onDelete,
}) {
  return (
    <Modal isActive={isActive} onCancel={onCancel}>
      <div className="box">
        <FieldInput
          label="Partial Name"
          placeholder="e.g. user-card"
          error={error}
          value={value}
          onChange={onChange}
          onSubmit={onSubmit}
        />
        <div className="field is-grouped">
          <ControlButton
            className="is-link"
            onClick={onSubmit}
            disabled={!!error}
          >
            Submit
          </ControlButton>
          <ControlButton className="is-link is-light" onClick={onCancel}>
            Cancel
          </ControlButton>
          <div className="is-flex-grow-1" />
          {isRename && (
            <ControlButton className="is-danger is-light" onClick={onDelete}>
              <Icon name="fa-times" />
              <span>Delete</span>
            </ControlButton>
          )}
        </div>
      </div>
    </Modal>
  );
}
