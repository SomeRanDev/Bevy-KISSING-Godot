(function() {
	const html = document.querySelector('html');
	const stylesheets = {
		ayuHighlight: document.querySelector('#mdbook-ayu-highlight-css'),
		tomorrowNight: document.querySelector('#mdbook-tomorrow-night-css'),
		highlight: document.querySelector('#mdbook-highlight-css'),
	};

	function refreshTheme() {
		setTimeout(function() {
			if(
				html.classList.contains("pinkrose") ||
				html.classList.contains("kanagawa") ||
				//html.classList.contains("burgundy") ||
				html.classList.contains("tokyonight")
			) {
				stylesheets.ayuHighlight.disabled = false;
				stylesheets.tomorrowNight.disabled = true;
				stylesheets.highlight.disabled = true;

				const ace_theme = "ace/theme/tomorrow_night";
				if(window.ace && window.editors) {
					window.editors.forEach(function (editor) {
						editor.setTheme(ace_theme);
					});
				}
			}
		});
	}

	const observer = new MutationObserver((mutations) => {
		for (const mutation of mutations) {
			if (mutation.type === "attributes" && mutation.attributeName === "class") {
				refreshTheme();
			}
		}
	});

	observer.observe(html, {
		attributes: true,
		attributeFilter: ["class"]
	});

	refreshTheme();
})();


