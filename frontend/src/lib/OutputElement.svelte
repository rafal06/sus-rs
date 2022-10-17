<script lang="ts">
    import { expandedUrls, type Url } from './store';

    import copyIcon from '/src/assets/copy.svg';
    import expandIcon from '/src/assets/expand.svg';

    export let urlObj: Url;
</script>

<div>
    <div class="output-box-text-div">
        <p id="_{urlObj.id}">{window.location.href + urlObj.id}</p>

        <div class="icons">
            <img src={copyIcon} alt="copy" class="hover-shadow copy-btn" data-clipboard-target="#_{urlObj.id}">

            <img src={expandIcon}
                alt="expand"
                class="hover-shadow expand-btn"
                class:expand-btn-collapsed={!!expandedUrls[urlObj.id]}
                on:click={() => expandedUrls[urlObj.id] = !expandedUrls[urlObj.id]}
            >
        </div>
    </div>

    <!-- If long url is expanded, show it -->
    {#if expandedUrls[urlObj.id]}
        <p class="full-url-p">{urlObj.url}</p>
    {/if}
</div>
