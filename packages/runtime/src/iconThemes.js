const Url = acode.require("url");

const details = {{ details }};

export default {
  init(firstInit, baseUrl) {
    for (const {id, name, root} of details) {
      window.vsApi.registerIconTheme(id, {
        name,
        rootUrl: Url.join(baseUrl, root),
        theme: async () => {
          return (await import(`./iconThemes/${id}.json`));
        },
        isMinimized: true,
        cssUrl: Url.join(this.baseUrl, "assets", `${id}.css`),
      });
    }

    if (firstInit) {
      editorManager.editor.commands.exec(
        "Preferences: File Icon Theme",
        editorManager.editor,
        undefined
      );
    }
  },

  dispose() {
    for (const detail of details) {
      window.vsApi.unRegisterIconTheme(detail.id);
    }
  }
};
