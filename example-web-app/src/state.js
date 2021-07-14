export const TEMPLATE = 0;

export function initialState() {
  return {
    activeTab: TEMPLATE,
    template: "<h1> Hello {{ labels.msg }} ! </h1>",
    data: '{ "labels": { "msg": "World" } }',
    partials: {},
    tabs: [],
    nameModal: initialModalState(),
    isPreview: true,
  };
}

export function initialModalState() {
  return {
    id: null,
    isActive: false,
    stagedId: "",
    error: null,
  };
}
