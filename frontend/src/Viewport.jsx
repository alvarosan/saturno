import * as React from 'react';

//import * as ctypes from 'ctypes';

import "./Viewport.css";

let wasm_promise = import("rendering_wasm")
    

export class Viewport extends React.Component {
    constructor(props) {
        super(props);

        this.renderers = new Map()

        this.state = {
            currentMode: props.mode,
            image: null,
            module: null,
            sceneId: 0,
        };

        wasm_promise.then(module => {
            this.state.module = module
        });

        this.renderingModeChanged = this.renderingModeChanged.bind(this);
        this.sceneIdChanged = this.sceneIdChanged.bind(this);
    }

    renderLocally() {

        // Need to import _bg to get a hold on the wasm memory buffer
        import ("rendering_wasm/rendering_wasm_bg.wasm").then(module => {
            if (!this.renderers.has(this.state.sceneId)) {
                const rend = this.state.module.create_renderer(this.state.sceneId)
                this.renderers.set(this.state.sceneId, rend)
            }

            //const frame = this.state.module.render()
            const frame = this.renderers.get(this.state.sceneId).render()

            const imageRaw = new Uint8ClampedArray(module.memory.buffer,
                frame.data(), frame.len())
            this.state.image = new ImageData(imageRaw, frame.width(), frame.height());
            const context = this.refs.local_canvas.getContext('2d');
            context.putImageData(this.state.image, 0, 0);
        });

        return ( <canvas className="viewport" ref="local_canvas"/> );
    }

    renderRemotely() {
        return ( <img className="viewport" src={ "/api/v1/render?sceneId=" + this.state.sceneId }/> );
    }

    renderingModeChanged(event) {
        this.setState({ currentMode:
            event.target.checked ? "locally" : "remotely" });
    }

    sceneIdChanged(event) {
        this.setState({ sceneId: event.target.value });
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
                <label for="rendering-mode">Local rendering</label>
                <input type="checkbox" name="rendering-mode" onChange={ this.renderingModeChanged }/>
            </div>
            <div>
                <label for="sceneId">Scene Id</label>
                <input type="text" name="sceneId" onChange={ this.sceneIdChanged } value={this.state.sceneId}/>
            </div>
            </div>
        );
    }
}


