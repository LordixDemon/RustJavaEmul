
export function rustjava_canvas2d_create(canvas, width, height) {
    const ctx = canvas.getContext("2d", { alpha: false, desynchronized: true });
    if (!ctx) {
        throw new Error("2d canvas context is not available");
    }
    ctx.imageSmoothingEnabled = false;
    return {
        canvas,
        ctx,
        width,
        height,
        imageData: ctx.createImageData(width, height),
    };
}

export function rustjava_canvas2d_resize(state, width, height) {
    state.width = width;
    state.height = height;
    state.imageData = state.ctx.createImageData(width, height);
}

export function rustjava_canvas2d_present(state, data) {
    if (state.imageData.data.length !== data.length) {
        throw new Error(`canvas2d buffer mismatch image=${state.imageData.data.length} data=${data.length}`);
    }
    state.imageData.data.set(data);
    state.ctx.putImageData(state.imageData, 0, 0);

    const canvasWidth = state.canvas.width;
    const canvasHeight = state.canvas.height;
    if (canvasWidth > state.width) {
        state.ctx.clearRect(state.width, 0, canvasWidth - state.width, canvasHeight);
    }
    if (canvasHeight > state.height) {
        state.ctx.clearRect(0, state.height, canvasWidth, canvasHeight - state.height);
    }
}
