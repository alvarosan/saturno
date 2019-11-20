import * as React from 'react';

import "./Viewport.css";

import * as Rend from "rendering_wasm";

export interface ViewportProps { mode: string; }

export class Viewport extends React.Component<ViewportProps, {}> {

    renderLocally() {
        const frame = Rend.render();

        return ( <div>Hello </div> );
    }


    renderRemotely() {
        return ( <img className="viewport" src="/api/v1/render"/> );
    }

    let cond = false;
    render() {
            <div className="wrapper">
                { cond ? this.renderRemotely() : this.renderLocally(); }
                <button className="u-btn" type="button">U</button> 
                <button className="d-btn" type="button">D</button> 
                <button className="l-btn" type="button">L</button> 
                <button className="r-btn" type="button">R</button> 
                <div>
                    Render time: 
                </div>
            </div>
    }
}


