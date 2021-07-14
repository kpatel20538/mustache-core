import { useState, useEffect } from "react";
import { render } from "mustache-wasm";

function useThrowableMemo(op, deps) {
  const [value, setValue] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    try {
      setValue(op());
      setError(null);
    } catch (e) {
      console.error(e);
      setError(e);
    }
  }, deps);

  return [value, error];
}

export function useMustache(template, data, partials) {
  const [context, jsonError] = useThrowableMemo(() => JSON.parse(data), [data]);

  const [output, templateError] = useThrowableMemo(
    () => render(template, context, (key) => partials[key] ?? ""),
    [template, context, partials]
  );

  return [output, jsonError ?? templateError ?? null];
}
