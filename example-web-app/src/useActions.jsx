import { useMemo, useState } from "react";

export function useActions(initialState, transitions) {
  const [state, setState] = useState(initialState);
  const actions = useMemo(
    () =>
      new Proxy(transitions, {
        get:
          (target, prop) =>
          (...args) =>
            setState(transitions[prop](...args)),
      }),
    [transitions]
  );

  return [state, actions];
}
