<script>
    import { invoke } from "@tauri-apps/api/core";

    let file = $state(null);
    let imageUrl = $state("");
    let image = $state("");
    let input_image = "";
    let quality = $state(8);
    let image_size = 1024;
    let message = $state("Process");
    let sorting_type = $state("Vibrancy");

    /**
     * @param {{ preventDefault: () => void; }} event
     */

    async function submit(event){
        if(file != null && message != "Processing..."){

            event.preventDefault();
            message = "Processing...";
            image = await invoke("process", {
                input: input_image,
                size: quality,
                sortType: sorting_type,
                outputSize: image_size,
            });
            message = "Process"
    
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
        } else {
            imageUrl = "";
        }
    };

    async function init() {
        const path = "src/routes/sowilo_logo@2x.png";
        const response = await fetch(path);
        const blob = await response.blob();
        imageUrl = URL.createObjectURL(blob);
    }

    init()

</script>

<main class="container">
    <!--
        
            
    
        </div>
        
        
        
        
        </div>
        -->

    <div class="row" style='height: 10vh;'>
        <div class="controlBar">
            <button class="submitButton controlItem" onclick={submit}>{message}</button>
            
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
            
            <div class="dropdown controlItem">
                <button class="hoverButton controlItem">{sorting_type}</button>
                <div class="content">
                    <button onclick={() => changeSortMode("Vibrancy")} class="dropButton controlItem">Vibrancy</button>
                    <button onclick={() => changeSortMode("Luminosity")} class="dropButton controlItem">Luminosity</button>
                    <button onclick={() => changeSortMode("Colorfulness")} class="dropButton controlItem">Colorfulness</button>
                    <button onclick={() => changeSortMode("Value")} class="dropButton controlItem">Value</button>
                    <button onclick={() => changeSortMode("Unsorted")} class="dropButton controlItem">Unsorted</button>
                </div>
            </div>
            
            <input class="controlItem" style="display: none" type="file" onchange={handleFileChange} accept="image/*" id="uploadButton"/>
            <label class="fileInput controlItem" for='uploadButton'>Select File</label>
    
        </div>
    </div>

    <div class="row" style='height: 90vh;'>
        <img class="image" src={imageUrl} alt="Sorter" />
    </div>
</main>

<style>

@font-face {
    font-family: 'Odin Rounded';
    src: url('src/routes/Odin Rounded - Bold.otf') format('opentype');
    font-weight: nomral;
    font-style: normal;
    font-display: swap;
}

:root {
    font-family: Odin Rounded, Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 20px;
    
    line-height: 24px;
    line-break: auto;
    font-weight: 500;
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
    overflow: hidden;
    
}

.image {
   max-height: 100%;
   max-width: 100%;

   width: auto;
   height: auto;
}

input,
button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.3em 0.6em;
    font-family: inherit;
    background-color: var(--button-color);
}

.controlItem {
    width: 25%;
    height: 100%;
    flex-shrink: 1;
    justify-content: center;
    align-items: center;
    font-size: min(4vh, 4vw);
}

.fileInput {
    border-radius: 8px;
    height: 100%;
    max-height: 100%;
    font-family: inherit;
    background-color: var(--button-color);
    justify-content: center;
    align-items: center;
    display: flex;
}

.fileInput:hover {
    border-color: rgb(0, 143, 129);
    background-color: var(--dark-button-color);
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
    border-radius: 16px;
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
    font-size: min(3vh, 3vw);
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

.container {
    padding: 0;
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
    margin: 0;
    padding: 0;
}

.controlBar {
    display: flex;
    box-sizing: border-box;
    padding: 1%;
    margin: 0;
    width: 100%;
    height: 100%;
}

input,

button {
    cursor: pointer;
}

button:hover {
    border-color: rgb(0, 143, 129);
    background-color: var(--dark-button-color);
}

button:active {
    border-color: rgb(0, 211, 190);
}

input,
button {
    outline: none;
}

</style>
