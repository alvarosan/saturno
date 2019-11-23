import * as React from 'react';

import "./Viewport.css";

import * as Rend from "rendering_wasm";
//const mod = import('rendering_wasm');

export interface ViewportProps { mode: string; }
export interface ViewportState { currentMode: string; }

export class Viewport extends React.Component<ViewportProps, ViewportState> {

    constructor(props: ViewportProps) {
        super(props);

        this.state = {
            currentMode: props.mode,
        };
    }

    renderLocally() {
        //        const mod = import('rendering_wasm').then(module => {
        //            const frame = module.render();
        //        });

//        import("rendering_wasm").then(module => {
//            const frame = module.render();
//        });

        return ( <div>Hello </div> );
    }


    renderRemotely() {
        return ( <img className="viewport" src="/api/v1/render"/> );
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
            Render time:
            </div>
            </div>
        );
    }
}


