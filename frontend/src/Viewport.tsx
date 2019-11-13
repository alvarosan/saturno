import * as React from 'react';

import "./Viewport.css";


export interface ViewportProps { mode: string; }

export class Viewport extends React.Component<ViewportProps, {}> {
    render() {
        return (
            <div className="wrapper">
                <img className="viewport" src="/api/v1/render"/>
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


