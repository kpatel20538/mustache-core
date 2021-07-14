import React from "react";
import CodeEditor from "./components/CodeEditor";
import CodePreview from "./components/CodePreview";
import FloatingActionButton from "./components/FloatingActionButton";
import Icon from "./components/Icon";
import NameModal from "./components/NameModal";
import TabBar from "./components/TabBar";
import { TEMPLATE, initialState } from "./state";
import * as transitions from "./transitions";
import { useActions } from "./useActions";
import { useMustache } from "./useMustache";
import "./styles.css";
import ErrorMessage from "./components/ErrorMessage";

export default function App() {
  const [state, actions] = useActions(initialState, transitions);
  const [output, error] = useMustache(
    state.template,
    state.data,
    state.partials
  );

  return (
    <div className="container">
      <TabBar
        active={state.activeTab}
        tabs={state.tabs}
        onView={(id) => actions.setActiveTab(id)}
        onRename={(id) => actions.openNameModal(id)}
        onAdd={() => actions.openNameModal()}
      />

      <div className="tile is-ancestor p-4">
        <div className="tile is-vertical is-6">
          <div className="tile is-vertical is-parent ">
            <CodeEditor
              className="tile is-child"
              value={
                state.activeTab === TEMPLATE
                  ? state.template
                  : state.partials[state.activeTab]
              }
              onChange={(value) => actions.setTabValue(value)}
            />
            <CodeEditor
              className="tile is-child"
              value={state.data}
              onChange={(value) => actions.setDataValue(value)}
            />
          </div>
        </div>
        <div className="tile is-6 is-parent">
          <div className="tile is-child">
            <div className="is-relative">
              <CodePreview isRichPreview={state.isPreview} value={output} />
              <FloatingActionButton onClick={() => actions.togglePreview()}>
                <Icon name={state.isPreview ? "fa-code" : "fa-eye"} />
              </FloatingActionButton>
            </div>
          </div>
        </div>
      </div>
      {error && <ErrorMessage>{error?.toString()}</ErrorMessage>}
      <NameModal
        value={state.nameModal.stagedId}
        error={state.nameModal.error}
        isActive={state.nameModal.isActive}
        isRename={!!state.nameModal.id}
        onChange={(text) => actions.setStagedId(text)}
        onCancel={() => actions.cancelNameModal()}
        onSubmit={() => actions.submitNameModal()}
        onDelete={() => actions.deletePartial()}
      />
    </div>
  );
}
