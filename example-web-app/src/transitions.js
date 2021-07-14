import produce from "immer";
import { initialModalState, TEMPLATE } from "./state";

export function setDataValue(value) {
  return produce((draft) => {
    draft.data = value;
  });
}

export function setTabValue(value) {
  return produce((draft) => {
    if (draft.activeTab === TEMPLATE) {
      draft.template = value;
    } else {
      draft.partials[draft.activeTab] = value;
    }
  });
}

export function setActiveTab(id) {
  return produce((draft) => {
    draft.activeTab = id;
  });
}

export function togglePreview() {
  return produce((draft) => {
    draft.isPreview = !draft.isPreview;
  });
}

export function openNameModal(id = null) {
  return produce((draft) => {
    draft.nameModal = {
      id,
      isActive: true,
      stagedId: id ?? "",
      error: null,
    };
  });
}

export function cancelNameModal() {
  return produce((draft) => {
    draft.nameModal = initialModalState();
  });
}

export function deletePartial() {
  return produce((draft) => {
    const id = draft.nameModal.id;
    const idx = draft.tabs.indexOf(id);

    delete draft.partials[id];
    draft.tabs.splice(idx, 1);

    draft.activeTab = idx === 0 ? TEMPLATE : draft.partials[draft.tabs[idx]];

    draft.nameModal = initialModalState();
  });
}

export function submitNameModal() {
  return produce((draft) => {
    draft.nameModal.error = getNameModalError(draft);
    if (draft.nameModal.error !== null) {
      return;
    }

    const id = draft.nameModal.id;
    const stagedId = draft.nameModal.stagedId.trim();
    if (id === null) {
      createPartial(draft, stagedId);
    } else if (id !== stagedId) {
      renamePartial(draft, id, stagedId);
    }

    draft.nameModal = initialModalState();
  });
}

export function setStagedId(stagedId) {
  return produce((draft) => {
    draft.nameModal.stagedId = stagedId;
    draft.nameModal.error = getNameModalError(draft);
  });
}

function getNameModalError(draft) {
  const id = draft.nameModal.id;
  const stagedId = draft.nameModal.stagedId.trim();

  if (stagedId === "") {
    return "Name is required";
  } else if (id !== stagedId && stagedId in draft.partials) {
    return "Name already in use";
  } else if (/\s/.test(stagedId)) {
    return "Name may not have spaces";
  } else {
    return null;
  }
}

function createPartial(draft, stagedId) {
  draft.partials[stagedId] = "{{! Empty Parital }}";
  draft.tabs.push(stagedId);
  draft.activeTab = stagedId;
}

function renamePartial(draft, id, stagedId) {
  const idx = draft.tabs.indexOf(id);
  draft.partials[stagedId] = draft.partials[id];
  draft.tabs.splice(idx, 1, stagedId);
  draft.activeTab = stagedId;
  delete draft.partials[id];
}
