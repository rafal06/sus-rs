<script lang="ts">
    import ClipboardJS from 'clipboard'

    import type { Url } from './lib/store';
    import { urlArr } from './lib/store';
    import copyIcon from '/src/assets/copy.svg';

    new ClipboardJS('.copy-btn');
    let urlBoxValue = '';

    async function shorten() {
        if(urlBoxValue === '') {
            return;
        }

        const response = fetch('.', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                url: urlBoxValue
            }),
        });

        const responseObj = (await response).json();

        const urlObj: Url = {
            id: (await responseObj).url,
            url: urlBoxValue,
        };

        // Insert urls to the Store
        urlArr.update(arr => {
            // New urls are inserted at the start of the array
            arr.unshift(urlObj);
            return arr;
        });

        urlBoxValue = '';
    }
</script>

<form class="shadow input-box">
    <input  id="url-box" class="hover-shadow" type="text" placeholder="Paste the URL..." bind:value={urlBoxValue}>
    <button id="btn-shorten" class="hover-shadow" on:click|preventDefault={shorten}>➞</button>
</form>

<div class="shadow output-box" class:output-shown={$urlArr.length !== 0}>
    <div>
        {#each $urlArr as urlObj}
            <div class="output-box-text-div">
                <p id="_{urlObj.id}">{window.location.href + urlObj.id}</p>
                <img src={copyIcon} alt="copy" class="hover-shadow copy-btn" data-clipboard-target="#_{urlObj.id}">
            </div>
        {/each}
    </div>
</div>
