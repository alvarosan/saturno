import * as React from 'react';

//import * as ctypes from 'ctypes';

import "./Viewport.css";

let wasm_promise = import("rendering_wasm")
    

export class Viewport extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            currentMode: props.mode,
            image: null,
            module: null,
        };

        wasm_promise.then(module => {
            this.state.module = module
        });

        this.renderingModeChanged = this.renderingModeChanged.bind(this);
    }

    renderLocally() {

        // Need to import _bg to get a hold on the wasm memory buffer
        import ("rendering_wasm/rendering_wasm_bg.wasm").then(module => {
            const frame = this.state.module.render()
            const imageRaw = new Uint8ClampedArray(module.memory.buffer,
                frame.data(), frame.len())
            this.state.image = new ImageData(imageRaw, frame.width(), frame.height());
            const context = this.refs.local_canvas.getContext('2d');
            context.putImageData(this.state.image, 0, 0);
        });

        return ( <canvas className="viewport" ref="local_canvas"/> );
    }

    renderRemotely() {
        return ( <img className="viewport" src="/api/v1/render"/> );
    }

    renderingModeChanged(event) {
        this.setState({ currentMode:
            event.target.checked ? "remotely" : "locally" });
    }

    render() {

        return (
            <div className="wrapper">
            { this.state.currentMode == "locally" ?
                this.renderLocally() :
                this.renderRemotely() }
            <button className="u-btn" type="button">U</button>
            <button className="d-btn" type="button">D</button>
            <button className="l-btn" type="button">L</button>
            <button className="r-btn" type="button">R</button>
            <div>
                <input type="checkbox" name="rendering-mode" onChange={ this.renderingModeChanged }/>
                <label for="rendering-mode">Remote rendering</label>
            </div>
            <div>
            Render time: 666.
            </div>
            </div>
        );
    }
}


