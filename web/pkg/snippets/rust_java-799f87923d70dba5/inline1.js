
export function rustjava_browser_micro_yield(callback) {
    const channel = new MessageChannel();
    channel.port1.onmessage = () => {
        channel.port1.close();
        channel.port2.close();
        callback();
    };
    channel.port2.postMessage(0);
}
