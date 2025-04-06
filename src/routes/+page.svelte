<script>
    import { invoke } from "@tauri-apps/api/core";

    let file = $state(null);
    let imageUrl = $state("");
    let image = $state("");
    let input_image = "";
    let quality = $state(8);
    let image_size = 1024;
    let sorting_type = $state("Vibrancy");

    /**
     * @param {{ preventDefault: () => void; }} event
     */

    async function submit(event){
        if(file != null){

            event.preventDefault();
            image = await invoke("process", {
                input: input_image,
                size: quality,
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

    const changeQuality = (/** @type {number} */ input) => {
        quality = input;
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
    <!--
        <div class="row" style='height: 20%;'>
            <div class="controlBar">
                <button class="submitButton controlItem" onclick={submit}>Process</button>
                
                <div class="dropdown controlItem">
                    <button class="hoverButton controlItem">{quality} x {quality}</button>
                    <div class="content">
                        <button onclick={() => changeQuality(1)} class="dropButton controlItem">1</button>
                        <button onclick={() => changeQuality(2)} class="dropButton controlItem">2</button>
                        <button onclick={() => changeQuality(4)} class="dropButton controlItem">4</button>
                        <button onclick={() => changeQuality(8)} class="dropButton controlItem">8</button>
                        <button onclick={() => changeQuality(16)} class="dropButton controlItem">16</button>
                        <button onclick={() => changeQuality(32)} class="dropButton controlItem">32</button>
                        <button onclick={() => changeQuality(64)} class="dropButton controlItem">64</button>
                        <button onclick={() => changeQuality(128)} class="dropButton controlItem">128</button>
                        <button onclick={() => changeQuality(256)} class="dropButton controlItem">256</button>
                    </div>
                </div>
                
                <input class="fileInput controlItem" type="file" onchange={handleFileChange} accept="image/*" />
                
                <div class="dropdown controlItem">
                    <button class="hoverButton controlItem">{sorting_type}</button>
                    <div class="content">
                        <button onclick={() => changeSortMode("Vibrancy")} class="dropButton controlItem">Vibrancy</button>
                        <button onclick={() => changeSortMode("Luminosity")} class="dropButton controlItem">Luminosity</button>
                        <button onclick={() => changeSortMode("Colorfulness")} class="dropButton controlItem">Colorfulness</button>
                        <button onclick={() => changeSortMode("Value")} class="dropButton controlItem">Value</button>
                        <button onclick={() => changeSortMode("Red Content")} class="dropButton controlItem">Red Content</button>
                        <button onclick={() => changeSortMode("Green Content")} class="dropButton controlItem">Green Content</button>
                        <button onclick={() => changeSortMode("Blue Content")} class="dropButton controlItem">Blue Content</button>
                        <button onclick={() => changeSortMode("Unsorted")} class="dropButton controlItem">Unsorted</button>
                    </div>
                </div>
        
            </div>
    
        </div>
        
        
        
        
        </div>
        -->
    <div class="row" style='height: 80%;'>
        <img class="image" src="src/routes/hue_sorted-2.png" alt="Sorter" />
    </div>
</main>

<style>

:root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    --button-color: rgb(0, 95, 86);
    --dark-button-color: rgb(0, 70, 64);
    --main-color: rgb(35, 31, 32);
    --dark-main-color: rgb(0, 37, 34);
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
    justify-content: center;
    align-items: center;
    position: relative;
}

.dropdown .hoverButton {
    background-color: var(--button-color);
    color: var(--text-color);
    width: 100%;
    height: 100%;
}

.content {
    display: none;
    position: absolute;
    background-color: var(--dark-main-color);
    padding: 5%;
    border: 2px solid transparent;
    border-radius: 8px;
    min-width: 100%;
    box-sizing: border-box;
}

.content .dropButton {
    width: 100%;
    box-shadow: 2px 2px 5px #000000ab;
    color: var(--text-color);
    background-color: var(--button-color);
    margin-top: 2%;
    box-sizing: border-box;
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
   max-height: 100vh;
   max-width: 100vw;
   width: auto;
   height: auto;
}

.imageContainer {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    overflow: hidden;
}

.container {
    margin: 0;
    width: 100%;
    height: 100%;
    max-width: 100%;
    max-height: 100%;
    display: block;
    flex-direction: column;
    justify-content: center;
    text-align: center;

}


.row {
    display: block;
    justify-content: center;
    width: 100%;
}

.controlBar {
    display: flex;
    margin: 0;
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
}

input,
button {
    outline: none;
}

</style>
