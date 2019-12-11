import * as React from 'react';

//import * as ctypes from 'ctypes';

import "./Viewport.css";

export class Viewport extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            currentMode: props.mode,
            image: null,
        };

        this.renderingModeChanged = this.renderingModeChanged.bind(this);
    }

    renderLocally() {
        import("rendering_wasm").then(module => {

            //module.greet();
            const frame = module.render()

            console.log(">>> module: ", module)
            console.log(">>> memory: ", module.wasm_memory.buffer)
//            console.log(">>> frame.data: ", frame.data())

// TODO This should help
//
//            https://www.hellorust.com/demos/canvas/index.html

//            const casted = ctypes.cast(frame.data(),
//                ctypes.uint8_t.array(frame.len()).ptr).contents
//
//            console.log(">>> casted: ", casted)

//            const buff = new ArrayBuffer(frame.len())
            const imageRaw = new Uint8ClampedArray(module.wasm_memory.buffer, // frame.data(),
                frame.data(), frame.len())

            console.log(">>> size, width, height: ", frame.len(), ", ", frame.width(),
                ", ", frame.height());

            console.log(">>> imageRaw: ", imageRaw);

            this.state.image = new ImageData(imageRaw, frame.width(), frame.height());
            const context = this.refs.local_canvas.getContext('2d');
            context.putImageData(this.state.image, 0, 0);

            console.log(">>> the image: ", this.state.image);
        });

        return ( <canvas ref="local_canvas"/> );
    }

    renderRemotely() {
        return ( <img className="viewport" src="/api/v1/render"/> );
    }

    renderingModeChanged(event) {
        console.log("->>> changing rendierng mode: ", this.state.currentMode);
        this.setState({ currentMode:
            event.target.checked ? "remotely" : "locally" });
        console.log("->>> changing rendierng mode after: ", this.state.currentMode);
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


