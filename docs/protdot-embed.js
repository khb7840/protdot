(function () {
    const queue = [];
    const encoder = new TextEncoder();
    const themeNames = [
        "Default",
        "Myth-Bursting",
        "Resistance To Love",
        "Attractive Vintage",
        "Desi Istyle",
        "Juggle The Rainbow",
        "Indian Marriage",
        "Cream Truck",
        "Fakes Are For Free",
        "Industrial Use",
        "Spring Sunset",
        "Alone With Thoughts",
        "Naughty Generation X",
        "Cleaning Wounds",
        "Another World",
        "Fire Mountain",
        "Beautiful Sundowners",
        "Bad For Economy",
        "Indian Drama",
        "Retro Base",
        "String Of Holidays",
    ];

    window.importObject = window.importObject || {};
    importObject.env = importObject.env || {};

    const statusNode = () => document.getElementById("status");

    function setStatus(message, isError = false) {
        const node = statusNode();
        if (!node) {
            return;
        }

        node.textContent = message;
        node.classList.toggle("error", isError);
    }

    function enqueue(command) {
        queue.push(command);
    }

    function peek() {
        return queue[0] || null;
    }

    function normalizeIndex(value, fallback = 0) {
        const index = Number(value);
        return Number.isFinite(index) ? index : fallback;
    }

    function updateRadiusValue(value) {
        const node = document.getElementById("radius-value");
        if (node) {
            node.textContent = `${Number(value).toFixed(1)}×`;
        }
    }

    function updateAnimationSpeedValue(value) {
        const node = document.getElementById("animation-speed-value");
        if (node) {
            node.textContent = `${Number(value).toFixed(1)}×`;
        }
    }

    importObject.env.protdot_command_pending = function () {
        return queue.length > 0 ? 1 : 0;
    };

    importObject.env.protdot_command_kind = function () {
        return peek() ? peek().kind : 0;
    };

    importObject.env.protdot_command_string_len = function () {
        const bytes = encoder.encode((peek() && peek().text) || "");
        return bytes.length;
    };

    importObject.env.protdot_command_string_copy = function (ptr, maxLen) {
        const bytes = encoder.encode((peek() && peek().text) || "");
        const view = new Uint8Array(wasm_memory.buffer, ptr, maxLen);
        view.fill(0);
        view.set(bytes.subarray(0, maxLen));
    };

    importObject.env.protdot_command_f32 = function () {
        return Number((peek() && peek().value) || 0);
    };

    importObject.env.protdot_command_u32 = function () {
        return Number((peek() && peek().value) || 0) >>> 0;
    };

    importObject.env.protdot_command_consume = function () {
        queue.shift();
    };

    const api = {
        themeNames,
        async loadPdbUrl(url) {
            const trimmedUrl = (url || "").trim();
            if (!trimmedUrl) {
                setStatus("Enter a PDB URL first.", true);
                return;
            }

            setStatus(`Fetching ${trimmedUrl}…`);

            try {
                const response = await fetch(trimmedUrl);
                if (!response.ok) {
                    throw new Error(`HTTP ${response.status}`);
                }

                const pdbText = await response.text();
                enqueue({ kind: 12, text: trimmedUrl });
                enqueue({ kind: 1, text: pdbText });
                setStatus(`Queued ${trimmedUrl}`);
                this.focus();
            } catch (error) {
                setStatus(`Failed to load URL: ${error.message}`, true);
            }
        },
        async loadFile(file) {
            if (!file) {
                setStatus("Choose a file first.", true);
                return;
            }

            try {
                const pdbText = await file.text();
                enqueue({ kind: 12, text: file.name || "Uploaded file" });
                enqueue({ kind: 1, text: pdbText });
                setStatus(`Queued ${file.name}`);
                this.focus();
            } catch (error) {
                setStatus(`Failed to read file: ${error.message}`, true);
            }
        },
        loadPdbText(text, label = "Browser input") {
            const pdbText = (text || "").trim();
            if (!pdbText) {
                setStatus("Paste PDB text before loading.", true);
                return;
            }

            enqueue({ kind: 12, text: label });
            enqueue({ kind: 1, text: pdbText });
            setStatus(`Queued ${label}`);
            this.focus();
        },
        loadDefault() {
            enqueue({ kind: 13 });
            setStatus("Queued embedded default structure");
            this.focus();
        },
        setColorScheme(index) {
            enqueue({ kind: 2, value: normalizeIndex(index) });
        },
        setRenderMode(index) {
            enqueue({ kind: 3, value: normalizeIndex(index) });
        },
        nextTheme() {
            enqueue({ kind: 4 });
        },
        previousTheme() {
            enqueue({ kind: 5 });
        },
        setThemeIndex(index) {
            enqueue({ kind: 14, value: normalizeIndex(index) });
        },
        setRadiusScale(value) {
            const radius = Math.max(0.1, Number(value));
            enqueue({ kind: 6, value: radius });
            updateRadiusValue(radius);
        },
        resetView() {
            enqueue({ kind: 7 });
        },
        setAnimationEnabled(enabled) {
            enqueue({ kind: 8, value: enabled ? 1 : 0 });
        },
        setAnimationMode(index) {
            enqueue({ kind: 9, value: normalizeIndex(index) });
        },
        setAnimationSpeed(value) {
            const speed = Math.max(0, Math.min(5, Number(value)));
            enqueue({ kind: 10, value: speed });
            updateAnimationSpeedValue(speed);
        },
        setUiVisible(visible) {
            enqueue({ kind: 11, value: visible ? 1 : 0 });
        },
        focus() {
            document.getElementById("glcanvas")?.focus();
        },
    };

    window.ProtdotViewer = api;
    window.createProtdotViewer = function () {
        return api;
    };

    document.addEventListener("DOMContentLoaded", function () {
        const themeSelect = document.getElementById("theme-select");
        const radiusScale = document.getElementById("radius-scale");
        const animationSpeed = document.getElementById("animation-speed");
        let overlayVisible = true;

        themeNames.forEach(function (name, index) {
            const option = document.createElement("option");
            option.value = String(index);
            option.textContent = name;
            themeSelect.appendChild(option);
        });

        document.getElementById("load-url").addEventListener("click", function () {
            api.loadPdbUrl(document.getElementById("pdb-url").value);
        });

        document.getElementById("pdb-url").addEventListener("keydown", function (event) {
            if (event.key === "Enter") {
                api.loadPdbUrl(event.currentTarget.value);
            }
        });

        document.getElementById("pdb-file").addEventListener("change", function (event) {
            api.loadFile(event.currentTarget.files[0]);
        });

        document.getElementById("load-default").addEventListener("click", function () {
            api.loadDefault();
        });

        document.getElementById("load-text").addEventListener("click", function () {
            api.loadPdbText(document.getElementById("pdb-text").value, "Pasted PDB text");
        });

        document.getElementById("color-scheme").addEventListener("change", function (event) {
            api.setColorScheme(event.currentTarget.value);
        });

        document.getElementById("render-mode").addEventListener("change", function (event) {
            api.setRenderMode(event.currentTarget.value);
        });

        document.getElementById("prev-theme").addEventListener("click", function () {
            const nextIndex = (themeSelect.selectedIndex - 1 + themeNames.length) % themeNames.length;
            themeSelect.selectedIndex = nextIndex;
            api.previousTheme();
        });

        document.getElementById("next-theme").addEventListener("click", function () {
            const nextIndex = (themeSelect.selectedIndex + 1) % themeNames.length;
            themeSelect.selectedIndex = nextIndex;
            api.nextTheme();
        });

        themeSelect.addEventListener("change", function (event) {
            api.setThemeIndex(event.currentTarget.value);
        });

        radiusScale.addEventListener("input", function (event) {
            api.setRadiusScale(event.currentTarget.value);
        });

        document.getElementById("reset-view").addEventListener("click", function () {
            api.resetView();
        });

        document.getElementById("toggle-overlay").addEventListener("click", function (event) {
            overlayVisible = !overlayVisible;
            api.setUiVisible(overlayVisible);
            event.currentTarget.textContent = overlayVisible ? "Hide overlay" : "Show overlay";
        });

        document.getElementById("animation-enabled").addEventListener("change", function (event) {
            api.setAnimationEnabled(event.currentTarget.value === "1");
        });

        document.getElementById("animation-mode").addEventListener("change", function (event) {
            api.setAnimationMode(event.currentTarget.value);
        });

        animationSpeed.addEventListener("input", function (event) {
            api.setAnimationSpeed(event.currentTarget.value);
        });

        updateRadiusValue(radiusScale.value);
        updateAnimationSpeedValue(animationSpeed.value);
        setStatus("Viewer ready. Load a URL, file, or pasted PDB text.");
    });
})();
