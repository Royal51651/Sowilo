<script>
    import { invoke } from "@tauri-apps/api/core";

    let file = $state(null);
    let imageUrl = $state("");
    let image = $state("");
    let input_image = "";
    let quality = 9;
    let image_size = 1024;
    let sorting_type = "luminosity";

    /**
     * @param {{ preventDefault: () => void; }} event
     */

    async function submit(event){
        event.preventDefault();
        var new_quality = 1 * Math.pow(2, quality - 1);
        console.log("New: " + new_quality.toString())
        image = await invoke("process", {
            input: input_image,
            size: new_quality,
            sortType: sorting_type,
            outputSize: image_size,
        });

        const bytes = atob(image);
        const byteArray1 = [];
        for (let i = 0; i < bytes.length; i++) {
            const byte = bytes.charCodeAt(i);
            byteArray1.push(byte);
        }
        const byteArray = new Uint8Array(byteArray1);
        const blob = new Blob([byteArray], { type: "image/png" });
        imageUrl = URL.createObjectURL(blob);
    }

    const handleFileChange = (
        /** @type {{ target: { files: null[]; }; }} */ e,
    ) => {
        file = e.target.files[0];
        if (file) {
            imageUrl = URL.createObjectURL(file);

            const reader = new FileReader();
            reader.onload = () => {
                // @ts-ignore
                input_image = reader.result.split(",")[1];
            };

            reader.readAsDataURL(file);
        }
    };


</script>

<main class="container">
    <button onclick={submit}>Process</button>
    <input type="range" bind:value={quality} min="1" max="9"/>
    <input type="file" onchange={handleFileChange} accept="image/*" />
    <img class="image" src={imageUrl} alt="Sorter" />

</main>

<style>

:root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
    color: #f6f6f6;
    background-color: #2f2f2f;
}

.image {
    width: 100%;
    height: 100%;
}

.container {
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
}

.logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
}

.logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
}

.row {
    display: flex;
    justify-content: center;
    }

a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
}

a:hover {
    color: #535bf2;
}

h1 {
    text-align: center;
}

input,
button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
    cursor: pointer;
}

button:hover {
    border-color: #396cd8;
}
button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
}

input,
button {
    outline: none;
}

#greet-input {
    margin-right: 5px; 
}

a:hover {
    color: #24c8db;
}

input,
button {
    color: #ffffff;
    background-color: #0f0f0f98;
}
button:active {
    background-color: #0f0f0f69;
}

</style>
