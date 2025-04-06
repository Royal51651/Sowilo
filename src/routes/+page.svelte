<script>
    import { invoke } from "@tauri-apps/api/core";

    let file = $state(null);
    let imageUrl = $state("");
    let image = $state("");
    let input_image = "";
    let quality = $state(1);
    let new_quality = $derived(1 * Math.pow(2, quality - 1));
    let image_size = 1024;
    let sorting_type = $state("Luminosity");

    /**
     * @param {{ preventDefault: () => void; }} event
     */

    async function submit(event){
        if(file != null){

            event.preventDefault();
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
    }

    const changeSortMode = (/** @type {string} */ input) => {
        sorting_type = input;
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
    <div class="controlBar">
        <button class="submitButton controlItem" onclick={submit}>Process at {new_quality} x {new_quality}</button>
        <input class="selectSlider controlItem" type="range" bind:value={quality} min="1" max="9"/>
        <input class="fileInput controlItem" type="file" onchange={handleFileChange} accept="image/*" />
        <div class="dropdown controlItem">
            <button class="hoverButton controlItem">{sorting_type}</button>
            <div class="content">
                <button onclick={() => changeSortMode("Luminosity")} class="dropButton controlItem">Luminosity</button>
                <button onclick={() => changeSortMode("Colorfulness")} class="dropButton controlItem">Colorfulness</button>
                <button onclick={() => changeSortMode("Value")} class="dropButton controlItem">Value</button>
                <button onclick={() => changeSortMode("Vibrancy")} class="dropButton controlItem">Vibrancy</button>
                <button onclick={() => changeSortMode("Red Content")} class="dropButton controlItem">Red Content</button>
                <button onclick={() => changeSortMode("Green Content")} class="dropButton controlItem">Green Content</button>
                <button onclick={() => changeSortMode("Blue Content")} class="dropButton controlItem">Blue Content</button>
                <button onclick={() => changeSortMode("Unsorted")} class="dropButton controlItem">Unsorted</button>
            </div>
        </div>

    </div>
    <img class="image" src={imageUrl} alt="Sorter" />

</main>

<style>

:root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    --button-color: rgb(0, 95, 86);
    --dark-button-color: rgb(0, 37, 34);
    --main-color: rgb(35, 31, 32);
    --text-color: rgb(255, 255, 255);
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
    color: #f6f6f6;
    background-color: var(--main-color);
    padding: 0%;
    margin: 0%;
    
}

input,
button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.3em 0.6em;
    font-weight: 500;
    font-family: inherit;
    background-color: #ffffff;
}

.controlItem {
    width: 25%;
    font-size: 1.2em;
}

.dropdown {
    display: inline-block;
}

.dropdown .hoverButton {
    background-color: var(--button-color);
    color: var(--text-color);
    width: 100%;
    height: 100%;
}

.content {
    display: block;
    text-decoration: none;
    position: absolute;
    display: none;
    background-color: var(--main-color);
    padding: 2%;
    border: 2px solid transparent;
    border-bottom-left-radius: 8px;
}

.content .dropButton {
    min-width: 100%;
    box-shadow: 2px 2px 5px #000000ab;
    color: var(--text-color);
    background-color: var(--button-color);
    margin-top: 2%;
}

.dropButton:hover {
    background-color: var(--dark-button-color);
}

.dropdown:hover .content{
    display: block;
}

.dropdown:hover .hoverButton {
    background-color: var(--dark-button-color);
}

.selectSlider {
    background: var(--button-color);
    color: var(--text-color);
}

.submitButton {
    background-color: var(--button-color);
    color: var(--text-color);
}

.fileInput {
    background-color: var(--button-color);
    color: var(--text-color);
    width: 15%;
}

.image {
    width: 100%;
    height: 100%;
}

.container {
    margin: 0;
    width: 100%;
    height: 100%;
    max-width: 100%;
    max-height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
}


.row {
    display: flex;
    justify-content: center;
    width: 100%;
}

.controlBar {
    display: flex;
    justify-content: space-between;
    width: 100%;
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
    cursor: pointer;
}

button:hover {
    border-color: rgb(0, 143, 129);
}

button:active {
    border-color: rgb(0, 211, 190);
    border-width: 3px;

}

input,
button {
    outline: none;
}

</style>
