<script>
    import {onMount} from "svelte";
    import {powWorkJs} from "../spow/js/spowJs";
    import {pow_work_wasm} from "../spow/wasm/spow-wasm";

    // test challenge - expected result:
    // 1:20:1702840338:ewxCYK+NgXCdPOKO:wMMz7qoedXxsWt+fOxMsWey/rI1u0UHAvmItuuS3oP0:230289
    //
    // On a test machine, this has been calculated in the backend in ~250ms
    // The wasm version on the same machine in ~680ms
    // The JS version in ~13600ms
    // const challenge = "1:20:1702840338:ewxCYK+NgXCdPOKO:wMMz7qoedXxsWt+fOxMsWey/rI1u0UHAvmItuuS3oP0:";

    // test challenge - expected result:
    // 1:14:1702763000:kBAxun85H3u8VU7V:Bo47e37phaYgu2fIpI7Pss4otpLglpLBI47KiirhWpI:39494
    // const challenge = "1:14:1702763000:kBAxun85H3u8VU7V:Bo47e37phaYgu2fIpI7Pss4otpLglpLBI47KiirhWpI:";

    // test challenge - expected result:
    // 1:18:1702982208:imxav82e3GFISa7i:VDv6pQsvH3DbtmZIHwNmZKcglw3j/FTn/BBcgwaOG/Q:321230
    const challenge = "1:18:1702982208:imxav82e3GFISa7i:VDv6pQsvH3DbtmZIHwNmZKcglw3j/FTn/BBcgwaOG/Q:";

    /** @type {string | undefined} */
    let resultWasm = '';
    let timeTakenWasm = 0;

    /** @type {string | undefined} */
    let resultJs = '';
    let timeTakenJs = 0;

    onMount(async () => {
        // calculate with wasm
        let now = new Date().getTime();
        resultWasm = await pow_work_wasm(challenge);
        timeTakenWasm = new Date().getTime() - now;
        console.log('result from wasm after: ' + timeTakenWasm);

        // calculate with js only
        now = new Date().getTime();
        resultJs = await powWorkJs(challenge);
        timeTakenJs = new Date().getTime() - now;
        console.log('result from JS after: ' + timeTakenJs);
        console.log(resultJs);
    });

</script>

<h3>Resolving challenge:</h3>
<p>
    {challenge}
</p>

<h3>WASM - calculated result:</h3>
<p>
    {#if resultWasm}
        {resultWasm}<br>
        Time taken: {timeTakenWasm} ms
    {/if}
</p>

<h3>JS - calculated result:</h3>
<p>
    {#if resultJs}
        {resultWasm}<br>
        Time taken: {timeTakenJs} ms
    {/if}
</p>
