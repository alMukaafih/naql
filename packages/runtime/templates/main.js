{% if include.icon_themes -%}
  import iconThemes from "./iconThemes";
{%- endif %}

const vscode = acode.require("vscode");

class Main {
  async init(firstInit) {
    {% if include.icon_themes -%}
      iconThemes.init(firstInit, this.baseUrl);
    {%- endif %}
  }

  reset() {
    {% if include.icon_themes -%}
      iconThemes.dispose();
    {%- endif %}
  }

  dispose() {
    this.reset();
  }
}

if (window.acode) {
  const main = new Main();
  acode.setPluginInit({{ id | js_string }}, async (baseUrl, _$page, { firstInit }) => {
    if (!baseUrl.endsWith("/")) {
      baseUrl += "/";
    }

    main.baseUrl = baseUrl;
    if (vscode && window.vsApi) {
      await main.init(firstInit);
    } else {
      window.addEventListener("vscode-api", async () => {
        await main.init(firstInit);
      });
    }
  });

  acode.setPluginUnmount({{ id | js_string }}, () => {
    main.dispose();
  });
}
