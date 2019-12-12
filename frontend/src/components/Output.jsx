import React from 'react';
import OutputProcess from "./OutputProcess";
import '../styles/Output.css';

class Output extends React.Component {
    render() {
        const {processes, pageFaultRate, no_processes, algorithm, interval, frames, method} = this.props;
        return (
            <div className="output">
                <div className="method-name">{method}</div>
                <div className="out-div">
                    <div className="head">
                        <div>
                            <span>frames:</span> <span>{Math.round(frames * 100) / 100}</span>
                        </div>
                        <div className="nop-div">
                            number of processes: {no_processes}
                        </div>
                        <div className="algorithm-div">
                            algorithm: {algorithm.toUpperCase()}
                        </div>
                        <div className="avg-pm-div">
                            avg page miss: {Math.round(pageFaultRate * 100) / 100}
                        </div>
                        <div className="interval-div">
                            interval: {interval}
                        </div>
                    </div>
                    {processes.map(process => (
                        <OutputProcess process={process}/>
                    ))}
                </div>
            </div>
        )
    }
}

export default Output;