{% if icon_themes %}
  import iconThemes from "./iconThemes";
{% endif %}

const vscode = acode.require("vscode");

class Main {
	async init(firstInit) {
    {% if icon_themes %}
      iconThemes.init(firstInit, this.baseUrl);
    {% endif %}
	}

	reset() {
    {% if icon_themes %}
      iconThemes.dispose();
    {% endif %}
	}

	dispose() {
		this.reset();
	}
}

if (window.acode) {
	const main = new Main();
	acode.setPluginInit({{ id }}, async (baseUrl, _$page, { firstInit }) => {
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

	acode.setPluginUnmount({{ id }}, () => {
		main.dispose();
	});
}
