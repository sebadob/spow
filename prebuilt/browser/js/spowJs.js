/**
 * Calculates the Proof of Work for the given challenge
 * @param {string} challenge
 * @return {Promise<string | undefined>} the calculated PoW
 */
export async function powWorkJs(challenge) {
    if (challenge.length < 5) {
        console.error('Invalid PoW challenge input length');
        return undefined;
    }

    const version = challenge[0];
    if (version !== '1') {
        console.error('Unknown PoW challenge version');
        return undefined;
    }

    const difficulty = parseInt(challenge.slice(2, 4));

    const sliceLength = (difficulty / 8) + 1;
    const bits = [128, 64, 32, 16, 8, 4, 2, 1];
    let counter = 0;

    while (true) {
        // combined value
        const value = challenge + counter;

        // sha256
        const data = new TextEncoder().encode(value);
        const hash = await crypto.subtle.digest("SHA-256", data);

        // create a view into the buffer
        const view = new Int8Array(hash.slice(0, sliceLength));

        // read the first bits until difficulty has been reached
        let bit = 0;
        while (bit < difficulty) {
            // This looks confusing, but it actually reads the bit from the given position
            let value = view[bit >> 3] & bits[bit % 8] ? 1 : 0;
            if (value === 1) {
                break;
            }
            bit += 1;
        }

        // if we have enough leading 0's -> success
        if (bit === difficulty) {
            return value;
        }

        counter += 1;
    }
}
